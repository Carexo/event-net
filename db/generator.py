import random
from datetime import datetime, timedelta

# Provided helper functions (assuming they work as intended)
def get_random_passed_date():
    """Generates a random datetime in the past (2020-2024)."""
    a = datetime(
        year=random.randint(2020, 2024),
        month=random.randint(1, 12),
        day=random.randint(1, 27), # Using 27 to avoid issues with months having less than 31 days
        hour=random.randint(1, 23),
        minute=0
    )

    return a

def get_random_future_date():
    """Generates a random datetime in the future (2026-2030)."""
    a = datetime(
        year=random.randint(2026, 2030),
        month=random.randint(1, 12),
        day=random.randint(1, 27), # Using 27 to avoid issues with months having less than 31 days
        hour=random.randint(1, 23),
        minute=0
    )

    return a

# Provided classes
class Event:
    def __init__(self, id, name, startDatetime):
        self.id = id
        self.name = name
        self.startDatetime = startDatetime
        self.keywords = [] # List of EventKeyword objects

    def add_keyword(self, keyword):
        if isinstance(keyword, EventKeyword):
            self.keywords.append(keyword)
        else:
            print(f"Warning: Tried to add non-EventKeyword object '{keyword}' to event '{self.name}'")

class EventKeyword:
    def __init__(self, name):
        self.name = name

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


# --- Data Generation ---

# 1. Create Keywords
keywords = {
    'rock_music': EventKeyword('Rock Music'),
    'pop_music': EventKeyword('Pop Music'),
    'electronic_music': EventKeyword('Electronic Music'),
    'jazz_music': EventKeyword('Jazz Music'),
    'classical_music': EventKeyword('Classical Music'),
    'folk_music': EventKeyword('Folk Music'), # Added
    'hip_hop_music': EventKeyword('Hip Hop Music'), # Added

    'football': EventKeyword('Football'),
    'basketball': EventKeyword('Basketball'),
    'tennis': EventKeyword('Tennis'),
    'volleyball': EventKeyword('Volleyball'),
    'swimming': EventKeyword('Swimming'),
    'cycling': EventKeyword('Cycling'), # Added
    'hiking': EventKeyword('Hiking'), # Added
    'running': EventKeyword('Running'), # Added

    'board_games': EventKeyword('Board Games'),
    'card_games': EventKeyword('Card Games'),
    'chess': EventKeyword('Chess'),
    'rpg': EventKeyword('RPG'),
    'puzzle_games': EventKeyword('Puzzle Games'), # Added

    'yoga': EventKeyword('Yoga'),
    'zumba': EventKeyword('Zumba'),
    'ballet': EventKeyword('Ballet'),
    'hip_hop_dance': EventKeyword('Hip Hop Dance'),
    'salsa': EventKeyword('Salsa'),
    'contemporary_dance': EventKeyword('Contemporary Dance'),
    'ballroom_dance': EventKeyword('Ballroom Dance'), # Added

    'music': EventKeyword('Music'),
    'sport': EventKeyword('Sport'),
    'workshop': EventKeyword('Workshop'),
    'competition': EventKeyword('Competition'),
    'live': EventKeyword('Live'),
    'match': EventKeyword('Match'),
    'class': EventKeyword('Class'),
    'tournament': EventKeyword('Tournament'),
    'festival': EventKeyword('Festival'),
    'social': EventKeyword('Social'),
    'online': EventKeyword('Online'),
    'exhibition': EventKeyword('Exhibition'),
    'tcg': EventKeyword('TCG'),
    'poker': EventKeyword('Poker'),
    'techno': EventKeyword('Techno'),
    'performance': EventKeyword('Performance'),
    'dance': EventKeyword('Dance'),
    'conference': EventKeyword('Conference'),

    'art': EventKeyword('Art'), # Added
    'photography': EventKeyword('Photography'), # Added
    'cooking': EventKeyword('Cooking'), # Added
    'writing': EventKeyword('Writing'), # Added
    'technology': EventKeyword('Technology'), # Added
}

# 2. Create Events
events = []

# Concerts (Past and Future)
event_id_counter = 1
def create_event(name, is_future, keywords_list):
    global event_id_counter
    date = get_random_future_date() if is_future else get_random_passed_date()
    event = Event(event_id_counter, name, date)
    for kw_key in keywords_list:
        if kw_key in keywords:
            event.add_keyword(keywords[kw_key])
    events.append(event)
    event_id_counter += 1
    return event

# Rock Concerts
rock_concert_past_1 = create_event("The Stone Roses Reunion", False, ['rock_music', 'live', 'music'])
rock_concert_past_2 = create_event("Arctic Monkeys Live 2023", False, ['rock_music', 'live', 'music'])
rock_concert_past_3 = create_event("Foo Fighters Concrete and Gold Tour", False, ['rock_music', 'live', 'music'])
rock_concert_past_4 = create_event("Queens of the Stone Age Villains Tour", False, ['rock_music', 'live', 'music']) # Added
rock_concert_future_1 = create_event("Future Rock Fest 2026", True, ['rock_music', 'live', 'festival', 'music'])
rock_concert_future_2 = create_event("Led Zeppelin Tribute Band", True, ['rock_music', 'live', 'music'])
rock_concert_future_3 = create_event("Greta Van Fleet Starcatcher Tour", True, ['rock_music', 'live', 'music'])
rock_concert_future_4 = create_event("The Black Keys Dropout Boogie Tour", True, ['rock_music', 'live', 'music']) # Added

# Pop Concerts
pop_concert_past_1 = create_event("Taylor Swift Eras Tour Warsaw", False, ['pop_music', 'live', 'music'])
pop_concert_past_2 = create_event("Dua Lipa Future Nostalgia Tour", False, ['pop_music', 'live', 'music'])
pop_concert_past_3 = create_event("Harry Styles Love on Tour", False, ['pop_music', 'live', 'music'])
pop_concert_future_1 = create_event("Billie Eilish World Tour 2027", True, ['pop_music', 'live', 'music'])
pop_concert_future_2 = create_event("Olivia Rodrigo GUTS Tour", True, ['pop_music', 'live', 'music'])
pop_concert_future_3 = create_event("Ed Sheeran +-=÷× Tour", True, ['pop_music', 'live', 'music']) # Added

# Electronic Music Events
electronic_event_past_1 = create_event("Tomorrowland Belgium 2022", False, ['electronic_music', 'live', 'festival', 'music'])
electronic_event_past_2 = create_event("EDC Las Vegas 2023", False, ['electronic_music', 'live', 'festival', 'music'])
electronic_event_future_1 = create_event("Ultra Music Festival Miami 2028", True, ['electronic_music', 'live', 'festival', 'music'])
electronic_event_future_2 = create_event("Awakenings Festival 2027", True, ['electronic_music', 'live', 'festival', 'techno', 'music'])
electronic_event_future_3 = create_event("Creamfields UK 2029", True, ['electronic_music', 'live', 'festival', 'music']) # Added

# Jazz & Classical Music Events
jazz_event_past_1 = create_event("Jazz on the Odra Festival 2023", False, ['jazz_music', 'live', 'festival', 'music'])
jazz_event_future_1 = create_event("Krakow Jazz Festival 2027", True, ['jazz_music', 'live', 'festival', 'music']) # Added
classical_event_future_1 = create_event("Warsaw Philharmonic Concert", True, ['classical_music', 'live', 'music'])
classical_event_future_2 = create_event("Vienna State Opera Performance", True, ['classical_music', 'live', 'performance', 'music']) # Added

# Folk & Hip Hop Music Events
folk_event_past_1 = create_event("Woodstock Poland 2021 (Pol'and'Rock)", False, ['folk_music', 'rock_music', 'live', 'festival', 'music']) # Added, intersection
hip_hop_event_future_1 = create_event("Kendrick Lamar The Big Steppers Tour", True, ['hip_hop_music', 'live', 'music']) # Added

# Sports Matches (Past and Future)
# Football
football_match_past_1 = create_event("Champions League Final 2023", False, ['football', 'match', 'competition', 'sport'])
football_match_past_2 = create_event("Premier League Derby", False, ['football', 'match', 'sport'])
football_match_past_3 = create_event("La Liga El Clasico", False, ['football', 'match', 'sport'])
football_match_past_4 = create_event("Serie A Milan Derby", False, ['football', 'match', 'sport']) # Added
football_match_future_1 = create_event("Euro 2028 Group Stage", True, ['football', 'match', 'competition', 'sport'])
football_match_future_2 = create_event("World Cup 2030 Final", True, ['football', 'match', 'competition', 'sport'])
football_match_future_3 = create_event("Copa America 2027 Final", True, ['football', 'match', 'competition', 'sport']) # Added

# Basketball
basketball_match_past_1 = create_event("NBA Finals Game 7 2024", False, ['basketball', 'match', 'competition', 'sport'])
basketball_match_past_2 = create_event("EuroLeague Championship", False, ['basketball', 'match', 'competition', 'sport'])
basketball_match_future_1 = create_event("FIBA World Cup 2027", True, ['basketball', 'match', 'competition', 'sport'])
basketball_match_future_2 = create_event("NCAA Final Four 2028", True, ['basketball', 'match', 'competition', 'sport']) # Added

# Tennis
tennis_match_past_1 = create_event("Wimbledon Men's Final 2023", False, ['tennis', 'match', 'competition', 'sport'])
tennis_match_past_2 = create_event("French Open Women's Semi-final", False, ['tennis', 'match', 'competition', 'sport'])
tennis_match_future_1 = create_event("Australian Open 2029", True, ['tennis', 'match', 'competition', 'sport'])
tennis_match_future_2 = create_event("US Open Men's Final 2027", True, ['tennis', 'match', 'competition', 'sport']) # Added

# Volleyball
volleyball_match_past_1 = create_event("FIVB World Championship Final", False, ['volleyball', 'match', 'competition', 'sport'])
volleyball_match_future_1 = create_event("Olympic Volleyball Tournament", True, ['volleyball', 'match', 'competition', 'sport'])

# Swimming, Cycling, Hiking, Running
swimming_event_future_1 = create_event("World Aquatics Championships", True, ['swimming', 'competition', 'sport'])
cycling_event_future_1 = create_event("Tour de France Stage 15", True, ['cycling', 'competition', 'sport']) # Added
hiking_event_future_1 = create_event("Guided Hike in the Tatras", True, ['hiking', 'social', 'outdoor', 'sport']) # Added outdoor keyword
running_event_future_1 = create_event("Warsaw Marathon 2027", True, ['running', 'competition', 'marathon', 'sport']) # Added marathon keyword

# Board Games
board_game_event_past_1 = create_event("Local Board Game Meetup", False, ['board_games', 'social'])
board_game_event_past_2 = create_event("Catan National Championship", False, ['board_games', 'tournament', 'competition'])
board_game_event_future_1 = create_event("International Board Game Convention", True, ['board_games', 'tournament', 'workshop'])
board_game_event_future_2 = create_event("Learn to Play Dungeons & Dragons", True, ['board_games', 'rpg', 'workshop'])
board_game_event_future_3 = create_event("Escape Room Challenge", True, ['puzzle_games', 'social']) # Added puzzle games keyword

# Card Games
card_game_event_past_1 = create_event("Poker Tournament Warsaw", False, ['card_games', 'poker', 'tournament'])
card_game_event_past_2 = create_event("Hearthstone Grandmasters", False, ['card_games', 'online', 'tournament'])
card_game_event_future_1 = create_event("Magic: The Gathering Friday Night Magic", True, ['card_games', 'tcg', 'tournament'])
card_game_event_future_2 = create_event("Bridge Club Meeting", True, ['card_games', 'social']) # Added

# Chess
chess_event_past_1 = create_event("Online Chess Blitz Tournament", False, ['chess', 'tournament', 'online'])
chess_event_future_1 = create_event("Grandmaster Chess Exhibition", True, ['chess', 'exhibition'])
chess_event_future_2 = create_event("Local Chess Club Meeting", True, ['chess', 'social'])

# Dance Classes & Events
dance_class_past_1 = create_event("Beginner Hip Hop Class", False, ['hip_hop_dance', 'class', 'dance'])
dance_class_past_2 = create_event("Intermediate Ballet Workshop", False, ['ballet', 'workshop', 'dance'])
dance_class_past_3 = create_event("Salsa Social Dance Night", False, ['salsa', 'social', 'dance'])
dance_class_future_1 = create_event("Zumba Fitness Session", True, ['zumba', 'class', 'dance'])
dance_class_future_2 = create_event("Advanced Salsa Class Series", True, ['salsa', 'class', 'dance'])
dance_class_future_3 = create_event("Contemporary Dance Performance", True, ['contemporary_dance', 'live', 'performance', 'dance'])
dance_class_future_4 = create_event("Beginner Ballroom Dance Class", True, ['ballroom_dance', 'class', 'dance']) # Added

# Yoga Classes
yoga_class_past_1 = create_event("Morning Vinyasa Flow", False, ['yoga', 'class'])
yoga_class_past_2 = create_event("Yoga for Beginners Workshop", False, ['yoga', 'workshop'])
yoga_class_future_1 = create_event("Evening Restorative Yoga", True, ['yoga', 'class'])
yoga_class_future_2 = create_event("Yoga Teacher Training Info Session", True, ['yoga', 'workshop'])
yoga_class_future_3 = create_event("Acro Yoga Jam Session", True, ['yoga', 'social']) # Added

# Other Workshops/Events
art_workshop_past_1 = create_event("Introduction to Naked Images (Porn)", False, ['art', 'workshop']) # Added
art_workshop_future_1 = create_event("Introduction to Oil Painting Workshop", True, ['art', 'workshop']) # Added
photography_walk_future_1 = create_event("Street Photography Photo Walk", True, ['photography', 'social', 'outdoor']) # Added
cooking_class_future_1 = create_event("Italian Cuisine Cooking Class", True, ['cooking', 'workshop']) # Added
writing_group_future_1 = create_event("Creative Writing Meetup", True, ['writing', 'social']) # Added
tech_conference_future_1 = create_event("Future of AI Conference 2028", True, ['technology', 'conference', 'workshop']) # Added conference keyword

# 3. Create Users
users = {
    'anna': User('Anna'), # Rock/Sports Fan
    'marek': User('Marek'), # Pop/Dance Fan
    'zosia': User('Zosia'), # Board/Card Game Fan
    'piotr': User('Piotr'), # Yoga/Electronic Music Fan
    'ewelina': User('Ewelina'), # Music Mix (Pop/Electronic) + Dance
    'krzysztof': User('Krzysztof'), # Sports Mix (Football/Volleyball) + Board Games
    'aga': User('Aga'), # Yoga + Classical Music + Board Games
    'tomek': User('Tomek'), # Rock + Folk + Hiking
    'karolina': User('Karolina'), # Pop + Hip Hop + Dance (Hip Hop)
    'michal': User('Michał'), # Electronic + Cycling + Photography
    'natalia': User('Natalia'), # Board Games + RPG + Cooking
    'bartek': User('Bartek'), # Chess + Card Games + Technology
}

# 4. Simulate User Registrations and Participations

# Anna (Rock/Sports Fan)
users['anna'].add_registered(rock_concert_past_1)
users['anna'].add_registered(rock_concert_past_2)
users['anna'].add_registered(football_match_past_1)
users['anna'].add_registered(basketball_match_past_1)
users['anna'].add_registered(rock_concert_future_1)
users['anna'].add_registered(football_match_future_1)
users['anna'].add_registered(rock_concert_past_3)
users['anna'].add_registered(football_match_past_3)
users['anna'].add_registered(rock_concert_past_4) # Added
users['anna'].add_registered(football_match_past_4) # Added
users['anna'].add_registered(rock_concert_future_4) # Added
users['anna'].add_registered(football_match_future_3) # Added

# Simulate participation for past events Anna registered for
for event in users['anna'].registered_to:
    users['anna'].add_participated(event)


# Marek (Pop/Dance Fan)
users['marek'].add_registered(pop_concert_past_1)
users['marek'].add_registered(pop_concert_past_2)
users['marek'].add_registered(dance_class_past_1)
users['marek'].add_registered(dance_class_past_2)
users['marek'].add_registered(pop_concert_future_1)
users['marek'].add_registered(dance_class_future_1)
users['marek'].add_registered(pop_concert_past_3)
users['marek'].add_registered(dance_class_past_3)
users['marek'].add_registered(pop_concert_future_3) # Added
users['marek'].add_registered(dance_class_future_4) # Added

# Simulate participation for past events Marek registered for
for event in users['marek'].registered_to:
    users['marek'].add_participated(event)

# Zosia (Board/Card Game Fan)
users['zosia'].add_registered(board_game_event_past_1)
users['zosia'].add_registered(card_game_event_past_1)
users['zosia'].add_registered(chess_event_past_1)
users['zosia'].add_registered(board_game_event_future_1)
users['zosia'].add_registered(card_game_event_future_1)
users['zosia'].add_registered(chess_event_future_1)
users['zosia'].add_registered(board_game_event_past_2)
users['zosia'].add_registered(card_game_event_past_2)
users['zosia'].add_registered(board_game_event_future_3) # Added
users['zosia'].add_registered(card_game_event_future_2) # Added

# Simulate participation for past events Zosia registered for
for event in users['zosia'].registered_to:
    users['zosia'].add_participated(event)

# Piotr (Yoga/Electronic Music Fan)
users['piotr'].add_registered(yoga_class_past_1)
users['piotr'].add_registered(electronic_event_past_1)
users['piotr'].add_registered(yoga_class_future_1)
users['piotr'].add_registered(electronic_event_future_1)
users['piotr'].add_registered(yoga_class_past_2)
users['piotr'].add_registered(electronic_event_past_2)
users['piotr'].add_registered(yoga_class_future_3) # Added
users['piotr'].add_registered(electronic_event_future_3) # Added

# Simulate participation for past events Piotr registered for
for event in users['piotr'].registered_to:
    users['piotr'].add_participated(event)

# Ewelina (Music Mix + Dance) - Intersection Interests
users['ewelina'].add_registered(pop_concert_past_2)
users['ewelina'].add_registered(electronic_event_past_1)
users['ewelina'].add_registered(dance_class_past_1)
users['ewelina'].add_registered(pop_concert_future_1)
users['ewelina'].add_registered(electronic_event_future_1)
users['ewelina'].add_registered(dance_class_future_2)
users['ewelina'].add_registered(pop_concert_future_2)
users['ewelina'].add_registered(dance_class_future_3)
users['ewelina'].add_registered(pop_concert_past_3) # Added
users['ewelina'].add_registered(dance_class_past_3) # Added

# Simulate participation for past events Ewelina registered for
for event in users['ewelina'].registered_to:
    users['ewelina'].add_participated(event)

# Krzysztof (Sports Mix + Board Games) - Intersection Interests
users['krzysztof'].add_registered(football_match_past_1)
users['krzysztof'].add_registered(volleyball_match_past_1)
users['krzysztof'].add_registered(board_game_event_past_1)
users['krzysztof'].add_registered(football_match_future_1)
users['krzysztof'].add_registered(volleyball_match_future_1)
users['krzysztof'].add_registered(board_game_event_future_1)
users['krzysztof'].add_registered(football_match_past_3)
users['krzysztof'].add_registered(board_game_event_past_2)
users['krzysztof'].add_registered(football_match_future_3) # Added
users['krzysztof'].add_registered(board_game_event_future_3) # Added

# Simulate participation for past events Krzysztof registered for
for event in users['krzysztof'].registered_to:
    users['krzysztof'].add_participated(event)

# Aga (Yoga + Classical Music + Board Games) - Intersection Interests
users['aga'].add_registered(yoga_class_past_1)
users['aga'].add_registered(board_game_event_past_1)
users['aga'].add_registered(classical_event_future_1)
users['aga'].add_registered(yoga_class_future_1)
users['aga'].add_registered(board_game_event_future_1)
users['aga'].add_registered(yoga_class_past_2)
users['aga'].add_registered(board_game_event_past_2)
users['aga'].add_registered(classical_event_future_2) # Added
users['aga'].add_registered(board_game_event_future_2) # Added

# Simulate participation for past events Aga registered for
for event in users['aga'].registered_to:
    users['aga'].add_participated(event)

# Tomek (Rock + Folk + Hiking) - New User with Intersection
users['tomek'].add_registered(rock_concert_past_1)
users['tomek'].add_registered(folk_event_past_1) # Intersection: Rock/Folk
users['tomek'].add_registered(rock_concert_future_1)
users['tomek'].add_registered(hiking_event_future_1) # New interest: Hiking
users['tomek'].add_registered(rock_concert_past_4) # Added

# Simulate participation for past events Tomek registered for
for event in users['tomek'].registered_to:
    users['tomek'].add_participated(event)

# Karolina (Pop + Hip Hop + Dance (Hip Hop)) - New User with Intersection
users['karolina'].add_registered(pop_concert_past_1)
users['karolina'].add_registered(dance_class_past_1) # Intersection: Pop/Hip Hop Dance
users['karolina'].add_registered(pop_concert_future_1)
users['karolina'].add_registered(hip_hop_event_future_1) # New interest: Hip Hop Music
users['karolina'].add_registered(pop_concert_future_2) # Added

# Simulate participation for past events Karolina registered for
for event in users['karolina'].registered_to:
    users['karolina'].add_participated(event)

# Michał (Electronic + Cycling + Photography) - New User with Intersection
users['michal'].add_registered(electronic_event_past_1)
users['michal'].add_registered(electronic_event_past_2) # Added
users['michal'].add_registered(electronic_event_future_1)
users['michal'].add_registered(cycling_event_future_1) # New interest: Cycling
users['michal'].add_registered(photography_walk_future_1) # New interest: Photography

# Simulate participation for past events Michał registered for
for event in users['michal'].registered_to:
    users['michal'].add_participated(event)

# Natalia (Board Games + RPG + Cooking) - New User with Intersection
users['natalia'].add_registered(board_game_event_past_1)
users['natalia'].add_registered(board_game_event_past_2) # Added
users['natalia'].add_registered(board_game_event_future_1)
users['natalia'].add_registered(board_game_event_future_2) # Intersection: Board Games/RPG
users['natalia'].add_registered(cooking_class_future_1) # New interest: Cooking

# Simulate participation for past events Natalia registered for
for event in users['natalia'].registered_to:
    users['natalia'].add_participated(event)

# Bartek (Chess + Card Games + Technology) - New User with Intersection
users['bartek'].add_registered(chess_event_past_1)
users['bartek'].add_registered(card_game_event_past_1)
users['bartek'].add_registered(chess_event_future_1)
users['bartek'].add_registered(card_game_event_future_1)
users['bartek'].add_registered(tech_conference_future_1) # New interest: Technology
users['bartek'].add_registered(chess_event_future_2) # Added

# Simulate participation for past events Bartek registered for
for event in users['bartek'].registered_to:
    users['bartek'].add_participated(event)


# --- Example of accessing the data ---

# Print all events
# print("--- All Events ---")
# for event in events:
#     keyword_names = [kw.name for kw in event.keywords]
#     print(f"ID: {event.id}, Name: {event.name}, Start: {event.startDatetime.strftime('%Y-%m-%d %H:%M')}, Keywords: {', '.join(keyword_names)}")

# Print user data
print("\n--- User Data ---")
for user_key, user in users.items():
    registered_event_names = [e.name for e in user.registered_to]
    participated_event_names = [e.name for e in user.participated_in]
    print(f"User: {user.name}")
    print(f"  Registered for: {', '.join(registered_event_names)}")
    print(f"  Participated in: {', '.join(participated_event_names)}")


# add events_registered_csv
events_registered_csv = []
for user_key, user in users.items():
    for e in user.registered_to:
        line = f'{user.name};{e.id};{e.name};{e.startDatetime.isoformat()}'
        events_registered_csv.append(line)

print(events_registered_csv)


# add keywords.csv
keywords_csv = []
for user_key, user in users.items():
    for e in user.registered_to:
        for k in e.keywords:
                line = f'{e.id};{k.name}'
                keywords_csv.append(line)


print(keywords_csv)