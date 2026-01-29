//! Event bus for broadcasting events to subscribers.
//!
//! The [`EventBus`] uses `tokio::broadcast` for efficient multi-producer,
//! multi-consumer event distribution. All subscribers receive all events.
//!
//! # Example
//!
//! ```ignore
//! use paramdef::event::{Event, EventBus};
//!
//! let bus = EventBus::new(64);
//!
//! // Subscribe before emitting
//! let mut sub = bus.subscribe();
//!
//! // Emit events
//! bus.emit(Event::touched("username"));
//!
//! // Receive events (async)
//! let event = sub.recv().await?;
//! ```

use std::sync::atomic::{AtomicU64, Ordering};

use tokio::sync::broadcast;

use super::types::Event;

/// Default channel capacity for the event bus.
pub const DEFAULT_CAPACITY: usize = 64;

/// Event bus for broadcasting parameter events.
///
/// Uses `tokio::broadcast` internally, which provides:
/// - Multi-producer, multi-consumer semantics
/// - All subscribers receive all events
/// - Bounded capacity with configurable lag handling
/// - Efficient cloning (subscribers share the channel)
///
/// # Capacity and Lag
///
/// The bus has a fixed capacity. If a subscriber falls behind by more than
/// the capacity, it will receive a `Lagged` error and miss some events.
/// Choose capacity based on expected event rate and subscriber processing speed.
///
/// # Thread Safety
///
/// `EventBus` is `Send + Sync` and can be shared across threads via `Arc`.
#[derive(Debug)]
pub struct EventBus {
    /// Broadcast sender for events.
    tx: broadcast::Sender<Event>,
    /// Counter for generating unique batch IDs.
    batch_counter: AtomicU64,
}

impl EventBus {
    /// Creates a new event bus with the specified capacity.
    ///
    /// # Arguments
    ///
    /// * `capacity` - Maximum number of events that can be buffered.
    ///   Subscribers that fall behind by more than this will miss events.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use paramdef::event::EventBus;
    ///
    /// let bus = EventBus::new(128);
    /// ```
    #[must_use]
    pub fn new(capacity: usize) -> Self {
        let (tx, _rx) = broadcast::channel(capacity);
        Self {
            tx,
            batch_counter: AtomicU64::new(0),
        }
    }

    /// Creates a new event bus with the default capacity (64).
    #[must_use]
    pub fn with_default_capacity() -> Self {
        Self::new(DEFAULT_CAPACITY)
    }

    /// Creates a subscription to receive events.
    ///
    /// The subscription will receive all events emitted after it was created.
    /// Events emitted before subscription are not received.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let bus = EventBus::new(64);
    /// let mut sub = bus.subscribe();
    ///
    /// bus.emit(Event::touched("field"));
    ///
    /// // sub will receive the event
    /// ```
    #[must_use]
    pub fn subscribe(&self) -> Subscription {
        Subscription {
            rx: self.tx.subscribe(),
        }
    }

    /// Emits an event to all subscribers.
    ///
    /// Returns the number of subscribers that received the event.
    /// Returns 0 if there are no active subscribers.
    ///
    /// This method never blocks - if the channel is full, the oldest
    /// event is dropped for lagging subscribers.
    pub fn emit(&self, event: Event) -> usize {
        self.tx.send(event).unwrap_or({
            // No active subscribers - event is intentionally dropped
            // This is expected behavior, not an error condition
            0
        })
    }

    /// Emits multiple events in sequence.
    ///
    /// Returns the total number of successful sends.
    pub fn emit_all(&self, events: impl IntoIterator<Item = Event>) -> usize {
        events.into_iter().map(|e| self.emit(e)).sum()
    }

    /// Returns the number of active subscribers.
    #[must_use]
    pub fn subscriber_count(&self) -> usize {
        self.tx.receiver_count()
    }

    /// Returns `true` if there are any active subscribers.
    #[must_use]
    pub fn has_subscribers(&self) -> bool {
        self.tx.receiver_count() > 0
    }

    /// Generates a unique batch ID.
    ///
    /// Used for `BatchBegin`/`BatchEnd` event pairs.
    #[must_use]
    pub fn next_batch_id(&self) -> u64 {
        self.batch_counter.fetch_add(1, Ordering::Relaxed)
    }

    /// Emits a `BatchBegin` event and returns the batch ID.
    ///
    /// Use [`end_batch`](Self::end_batch) with the returned ID to close the batch.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let batch_id = bus.begin_batch(Some("Updating user profile"));
    /// // ... emit individual events ...
    /// bus.end_batch(batch_id);
    /// ```
    pub fn begin_batch(&self, description: Option<impl Into<crate::core::SmartStr>>) -> u64 {
        let id = self.next_batch_id();
        self.emit(Event::batch_begin(id, description));
        id
    }

    /// Emits a `BatchEnd` event for the given batch ID.
    ///
    /// # Parameters
    ///
    /// - `id`: The batch ID from `begin_batch()`
    /// - `success`: Whether all operations in the batch succeeded
    /// - `partial`: Whether some operations succeeded (only relevant when success=false)
    pub fn end_batch(&self, id: u64, success: bool, partial: bool) {
        self.emit(Event::batch_end(id, success, partial));
    }

    /// Executes a closure within a batch, automatically emitting
    /// `BatchBegin` and `BatchEnd` events.
    ///
    /// The batch is considered successful if the closure completes without panicking.
    ///
    /// # Example
    ///
    /// ```ignore
    /// bus.batch(Some("Update multiple fields"), || {
    ///     ctx.set("name", Value::text("Alice"));
    ///     ctx.set("age", Value::Int(30));
    /// });
    /// ```
    pub fn batch<F, R>(&self, description: Option<impl Into<crate::core::SmartStr>>, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        let id = self.begin_batch(description);
        let result = f();
        self.end_batch(id, true, false); // success=true, partial=false
        result
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::with_default_capacity()
    }
}

impl Clone for EventBus {
    fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone(),
            batch_counter: AtomicU64::new(self.batch_counter.load(Ordering::Relaxed)),
        }
    }
}

/// A subscription to receive events from an [`EventBus`].
///
/// Created via [`EventBus::subscribe`]. Automatically unsubscribes when dropped.
///
/// # Async Usage
///
/// ```ignore
/// let mut sub = bus.subscribe();
///
/// loop {
///     match sub.recv().await {
///         Ok(event) => handle_event(event),
///         Err(RecvError::Lagged(n)) => println!("Missed {n} events"),
///         Err(RecvError::Closed) => break,
///     }
/// }
/// ```
///
/// # Sync Usage
///
/// ```ignore
/// // Non-blocking try_recv
/// while let Ok(event) = sub.try_recv() {
///     handle_event(event);
/// }
/// ```
#[derive(Debug)]
pub struct Subscription {
    rx: broadcast::Receiver<Event>,
}

impl Subscription {
    /// Receives the next event, waiting if necessary.
    ///
    /// # Errors
    ///
    /// - [`RecvError::Closed`] - The event bus was dropped
    /// - [`RecvError::Lagged`] - Missed events due to slow processing
    pub async fn recv(&mut self) -> Result<Event, RecvError> {
        self.rx.recv().await.map_err(|e| match e {
            broadcast::error::RecvError::Closed => RecvError::Closed,
            broadcast::error::RecvError::Lagged(n) => RecvError::Lagged(n),
        })
    }

    /// Tries to receive an event without waiting.
    ///
    /// Returns `None` if no event is available.
    ///
    /// # Errors
    ///
    /// - [`RecvError::Closed`] - The event bus was dropped
    /// - [`RecvError::Lagged`] - Missed events
    pub fn try_recv(&mut self) -> Result<Option<Event>, RecvError> {
        match self.rx.try_recv() {
            Ok(event) => Ok(Some(event)),
            Err(broadcast::error::TryRecvError::Empty) => Ok(None),
            Err(broadcast::error::TryRecvError::Closed) => Err(RecvError::Closed),
            Err(broadcast::error::TryRecvError::Lagged(n)) => Err(RecvError::Lagged(n)),
        }
    }

    /// Returns `true` if the event bus has been dropped.
    #[must_use]
    pub fn is_closed(&self) -> bool {
        // We can check by trying to receive - but that consumes events
        // Instead, we expose this through the len check
        self.rx.is_empty() && self.try_recv_peek_closed()
    }

    /// Internal helper to check if closed without consuming.
    fn try_recv_peek_closed(&self) -> bool {
        // Clone receiver to peek without consuming
        let mut peek = self.rx.resubscribe();
        matches!(peek.try_recv(), Err(broadcast::error::TryRecvError::Closed))
    }

    /// Returns the number of events waiting to be received.
    #[must_use]
    pub fn len(&self) -> usize {
        self.rx.len()
    }

    /// Returns `true` if there are no pending events.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.rx.len() == 0
    }

    /// Resubscribes to the event bus.
    ///
    /// Creates a new subscription that will receive events from this point forward.
    /// Any events currently buffered in this subscription will be skipped.
    #[must_use]
    pub fn resubscribe(&self) -> Self {
        Self {
            rx: self.rx.resubscribe(),
        }
    }
}

/// Errors that can occur when receiving events.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecvError {
    /// The event bus was dropped and no more events will be sent.
    Closed,
    /// The receiver lagged behind and missed events.
    ///
    /// Contains the number of missed events. The receiver can continue
    /// receiving subsequent events.
    Lagged(u64),
}

impl std::fmt::Display for RecvError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Closed => write!(f, "event bus closed"),
            Self::Lagged(n) => write!(f, "receiver lagged, missed {n} events"),
        }
    }
}

impl std::error::Error for RecvError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Value;
    use std::sync::Arc;

    #[test]
    fn test_event_bus_new() {
        let bus = EventBus::new(32);
        assert_eq!(bus.subscriber_count(), 0);
        assert!(!bus.has_subscribers());
    }

    #[test]
    fn test_event_bus_default() {
        let bus = EventBus::default();
        assert_eq!(bus.subscriber_count(), 0);
    }

    #[test]
    fn test_subscriber_count() {
        let bus = EventBus::new(32);

        let _sub1 = bus.subscribe();
        assert_eq!(bus.subscriber_count(), 1);

        let _sub2 = bus.subscribe();
        assert_eq!(bus.subscriber_count(), 2);

        drop(_sub1);
        assert_eq!(bus.subscriber_count(), 1);
    }

    #[test]
    fn test_emit_no_subscribers() {
        let bus = EventBus::new(32);
        let count = bus.emit(Event::touched("key"));
        assert_eq!(count, 0);
    }

    #[test]
    fn test_batch_id_generation() {
        let bus = EventBus::new(32);

        let id1 = bus.next_batch_id();
        let id2 = bus.next_batch_id();
        let id3 = bus.next_batch_id();

        assert_eq!(id1, 0);
        assert_eq!(id2, 1);
        assert_eq!(id3, 2);
    }

    #[test]
    fn test_event_bus_clone() {
        let bus1 = EventBus::new(32);
        let _sub = bus1.subscribe();

        let bus2 = bus1.clone();

        // Both should share the same channel
        assert_eq!(bus1.subscriber_count(), 1);
        assert_eq!(bus2.subscriber_count(), 1);

        // Emit on one, received on subscriber
        bus2.emit(Event::touched("key"));
    }

    #[tokio::test]
    async fn test_subscription_recv() {
        let bus = EventBus::new(32);
        let mut sub = bus.subscribe();

        bus.emit(Event::touched("field1"));
        bus.emit(Event::dirtied("field2"));

        let e1 = sub.recv().await.unwrap();
        let e2 = sub.recv().await.unwrap();

        assert!(matches!(e1, Event::Touched { .. }));
        assert!(matches!(e2, Event::Dirtied { .. }));
    }

    #[tokio::test]
    async fn test_subscription_try_recv() {
        let bus = EventBus::new(32);
        let mut sub = bus.subscribe();

        // No events yet
        assert!(sub.try_recv().unwrap().is_none());

        bus.emit(Event::touched("field"));

        // Now there's an event
        let event = sub.try_recv().unwrap();
        assert!(event.is_some());

        // No more events
        assert!(sub.try_recv().unwrap().is_none());
    }

    #[tokio::test]
    async fn test_subscription_len() {
        let bus = EventBus::new(32);
        let sub = bus.subscribe();

        assert!(sub.is_empty());
        assert_eq!(sub.len(), 0);

        bus.emit(Event::touched("a"));
        bus.emit(Event::touched("b"));

        assert!(!sub.is_empty());
        assert_eq!(sub.len(), 2);
    }

    #[tokio::test]
    async fn test_batch_helper() {
        let bus = EventBus::new(32);
        let mut sub = bus.subscribe();

        let result = bus.batch(Some("test batch"), || {
            bus.emit(Event::touched("field"));
            42
        });

        assert_eq!(result, 42);

        // Should receive: BatchBegin, Touched, BatchEnd
        let e1 = sub.recv().await.unwrap();
        let e2 = sub.recv().await.unwrap();
        let e3 = sub.recv().await.unwrap();

        assert!(matches!(e1, Event::BatchBegin { id: 0, .. }));
        assert!(matches!(e2, Event::Touched { .. }));
        assert!(matches!(e3, Event::BatchEnd { id: 0, .. }));
    }

    #[tokio::test]
    async fn test_emit_all() {
        let bus = EventBus::new(32);
        let sub = bus.subscribe();

        let events = vec![
            Event::touched("a"),
            Event::touched("b"),
            Event::touched("c"),
        ];

        let count = bus.emit_all(events);
        assert_eq!(count, 3);

        assert_eq!(sub.len(), 3);
    }

    #[tokio::test]
    async fn test_closed_error() {
        let bus = EventBus::new(32);
        let mut sub = bus.subscribe();

        drop(bus);

        let result = sub.recv().await;
        assert!(matches!(result, Err(RecvError::Closed)));
    }

    #[tokio::test]
    async fn test_lagged_error() {
        let bus = EventBus::new(2); // Very small capacity
        let mut sub = bus.subscribe();

        // Emit more events than capacity
        for i in 0..10 {
            bus.emit(Event::value_changed(
                format!("key{i}"),
                None,
                Arc::new(Value::Int(i)),
            ));
        }

        // First recv should report lag
        let result = sub.recv().await;
        assert!(matches!(result, Err(RecvError::Lagged(_))));
    }

    #[test]
    fn test_resubscribe() {
        let bus = EventBus::new(32);
        let sub1 = bus.subscribe();

        bus.emit(Event::touched("before"));

        let sub2 = sub1.resubscribe();

        bus.emit(Event::touched("after"));

        // sub1 has both events
        assert_eq!(sub1.len(), 2);

        // sub2 only has events after resubscribe
        assert_eq!(sub2.len(), 1);
    }

    #[test]
    fn test_recv_error_display() {
        assert_eq!(RecvError::Closed.to_string(), "event bus closed");
        assert_eq!(
            RecvError::Lagged(5).to_string(),
            "receiver lagged, missed 5 events"
        );
    }

    #[test]
    fn test_event_bus_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<EventBus>();
        assert_send_sync::<Subscription>();
    }
}
