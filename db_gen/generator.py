from faker import Faker
from events import events
from User import User
import random

# --- Data Generation ---

events_types = sorted(events.keys())

def get_random_events():
    user_events_types = random.sample(events_types, random.randint(1, 3))
    user_events = []
    for event_type in user_events_types:
        user_events = random.choices(events[event_type]['past'], k=random.randint(1, len(events[event_type]['past']))) + \
            random.choices(events[event_type]['future'], k=random.randint(1, min(3, len(events[event_type]['future']))))
    return user_events

faker = Faker()
unique_usernames = set([faker.user_name() for _ in range(10000)])
users = {}
for username in unique_usernames:
    user = User(username)
    users[user.name] = user
    user_events = get_random_events()
    for e in user_events:
        user.add_registered(e)

COLUMNS_DELIMITER = ','
LINE_DELIMITER = '\n'

events_registered_csv = []
csv_header = ['user_name', 'event_id', 'event_name', 'start_datetime']
events_registered_csv.append(COLUMNS_DELIMITER.join(csv_header))
for user in users.values():
    for e in user.registered_to:
        props = [user.name, e.id, e.name, e.startDatetime.isoformat()]
        line = COLUMNS_DELIMITER.join(map(str, props))
        events_registered_csv.append(line)

with open('user_events.csv', 'w', encoding='utf-8') as f:
    f.write(LINE_DELIMITER.join(events_registered_csv))

flat_events = sum(sum([list(e.values()) for e in events.values()], []), [])

keywords_csv = []
csv_header = ['event_id', 'keyword']
keywords_csv.append(COLUMNS_DELIMITER.join(csv_header))
for e in flat_events:
    for k in e.keywords:
        props = [e.id, k]
        line = COLUMNS_DELIMITER.join(map(str, props))
        keywords_csv.append(line)

with open('event_keyword.csv', 'w', encoding='utf-8') as f:
    f.write(LINE_DELIMITER.join(keywords_csv))
