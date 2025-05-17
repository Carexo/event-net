from Event import Event
from datetime import datetime

class User:
    def __init__(self, name):
        self.name = name
        self.registered_to = [] # List of Event objects
        self.participated_in = [] # List of Event objects
    def add_registered(self, event):
        if isinstance(event, Event) and event not in self.registered_to:
            self.registered_to.append(event)
        elif not isinstance(event, Event):
             print(f"Warning: Tried to register non-Event object '{event}' for user '{self.name}'")

    def add_participated(self, event):
        # Assuming participation means they were registered and the event is in the past
        if isinstance(event, Event) and event in self.registered_to and event.startDatetime < datetime.now() and event not in self.participated_in:
             self.participated_in.append(event)
        elif not isinstance(event, Event):
             print(f"Warning: Tried to add non-Event object '{event}' to participated_in for user '{self.name}'")
        elif event not in self.registered_to:
             print(f"Warning: Event '{event.name}' not in registered_to for user '{self.name}'. Cannot add to participated_in.")
        elif event.startDatetime >= datetime.now():
             print(f"Warning: Event '{event.name}' is in the future. Cannot add to participated_in for user '{self.name}'.")
