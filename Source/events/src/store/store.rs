trait EventStore: EventCommitter + CommittedEventFetcher {}

trait EventCommitter {}

trait CommittedEventFetcher {}
