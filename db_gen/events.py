import random
from datetime import datetime
from Event import Event

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

event_id_counter = 1
def create_event(name, is_future, keywords_list):
    global event_id_counter
    date = get_random_future_date() if is_future else get_random_passed_date()
    event = Event(event_id_counter, name, date)
    for kw_key in keywords_list:
        event.add_keyword(kw_key)
    event_id_counter += 1
    return event


events = {}

# Rock Concerts
events['rock_concerts'] = {
    'past': [
        create_event("The Stone Roses Reunion", False, ['rock_music', 'live', 'music']),
        create_event("Arctic Monkeys Live 2023", False, ['rock_music', 'live', 'music']),
        create_event("Foo Fighters Concrete and Gold Tour", False, ['rock_music', 'live', 'music']),
        create_event("Queens of the Stone Age Villains Tour", False, ['rock_music', 'live', 'music']),
    ],
    'future': [
        create_event("Future Rock Fest 2026", True, ['rock_music', 'live', 'festival', 'music']),
        create_event("Led Zeppelin Tribute Band", True, ['rock_music', 'live', 'music']),
        create_event("Greta Van Fleet Starcatcher Tour", True, ['rock_music', 'live', 'music']),
        create_event("The Black Keys Dropout Boogie Tour", True, ['rock_music', 'live', 'music']),
    ]
}

# Pop Concerts
events['pop_concerts'] = {
    'past': [
        create_event("Taylor Swift Eras Tour Warsaw", False, ['pop_music', 'live', 'music']),
        create_event("Dua Lipa Future Nostalgia Tour", False, ['pop_music', 'live', 'music']),
        create_event("Harry Styles Love on Tour", False, ['pop_music', 'live', 'music']),
        create_event("Ariana Grande Dangerous", False, ['pop_music', 'live', 'music']),
        create_event("Justin Bieber Purpose World Tour", False, ['pop_music', 'live', 'music']),
        create_event("Demi Lovato Tell Me You Love Me Tour", False, ['pop_music', 'live', 'music']),
        create_event("Shawn Mendes Illuminate World Tour", False, ['pop_music', 'live', 'music']),
    ],
    'future': [
        create_event("Billie Eilish World Tour 2027", True, ['pop_music', 'live', 'music']),
        create_event("Olivia Rodrigo GUTS Tour", True, ['pop_music', 'live', 'music']),
        create_event("Ed Sheeran +-=÷× Tour", True, ['pop_music', 'live', 'music']),
        create_event("Ariana Grande Sweetener World Tour", True, ['pop_music', 'live', 'music']),
        create_event("The Weeknd After Hours Til Dawn Tour", True, ['pop_music', 'live', 'music']),
        create_event("Justin Bieber Justice World Tour", True, ['pop_music', 'live', 'music']),
        create_event("Demi Lovato Holy Fvck Tour", True, ['pop_music', 'live', 'music']),
        create_event("Shawn Mendes Wonder World Tour", True, ['pop_music', 'live', 'music']),
    ]
}

# Elec Concerts
events['electronic_events'] = {
    'past': [
        create_event("Tomorrowland Belgium 2022", False, ['electronic_music', 'live', 'festival', 'music']),
        create_event("EDC Las Vegas 2023", False, ['electronic_music', 'live', 'festival', 'music'])
    ],
    'future': [
        create_event("Ultra Music Festival Miami 2028", True, ['electronic_music', 'live', 'festival', 'music']),
        create_event("Awakenings Festival 2027", True, ['electronic_music', 'live', 'festival', 'techno', 'music']),
        create_event("Creamfields UK 2029", True, ['electronic_music', 'live', 'festival', 'music'])
    ]
}

# Sports Matches (Past and Future)
# Football
events['football_matches'] = {
    'past': [
        create_event("Champions League Final 2023", False, ['football', 'match', 'competition', 'sport']),
        create_event("Premier League Derby", False, ['football', 'match', 'sport']),
        create_event("La Liga El Clasico", False, ['football', 'match', 'sport']),
        create_event("Serie A Milan Derby", False, ['football', 'match', 'sport']),
        create_event("Bundesliga Der Klassiker", False, ['football', 'match', 'sport']),
        create_event("MLS All-Star Game", False, ['football', 'match', 'sport']),
        create_event("Ligue 1 Paris Derby", False, ['football', 'match', 'sport']),
        create_event("Eredivisie De Topper", False, ['football', 'match', 'sport']),
        create_event("Primeira Liga O Clássico", False, ['football', 'match', 'sport']),
        create_event("Scottish Premiership Old Firm", False, ['football', 'match', 'sport']),
        create_event("Super Lig Istanbul Derby", False, ['football', 'match', 'sport']),
        create_event("A-League Sydney Derby", False, ['football', 'match', 'sport']),
        create_event("J-League Tokyo Derby", False, ['football', 'match', 'sport']),
        create_event("MLS Eastern Conference Final", False, ['football', 'match', 'sport']),
        create_event("MLS Western Conference Final", False, ['football', 'match', 'sport']),
        create_event("MLS Cup Final", False, ['football', 'match', 'sport']),
        create_event("CONCACAF Champions League Final", False, ['football', 'match', 'sport']),
    ],
    'future': [
        create_event("Euro 2028 Group Stage", True, ['football', 'match', 'competition', 'sport']),
        create_event("World Cup 2030 Final", True, ['football', 'match', 'competition', 'sport']),
        create_event("Copa America 2027 Final", True, ['football', 'match', 'competition', 'sport']),
        create_event("AFCON 2027 Final", True, ['football', 'match', 'competition', 'sport']),
        create_event("AFC Asian Cup 2027 Final", True, ['football', 'match', 'competition', 'sport']),
        create_event("FIFA Club World Cup Final", True, ['football', 'match', 'competition', 'sport']),
        create_event("UEFA Super Cup", True, ['football', 'match', 'competition', 'sport']),
        create_event("CONCACAF Gold Cup Final", True, ['football', 'match', 'competition', 'sport']),
        create_event("CONMEBOL Libertadores Final", True, ['football', 'match', 'competition', 'sport']),
        create_event("CONMEBOL Sudamericana Final", True, ['football', 'match', 'competition', 'sport']),
        create_event("UEFA Europa League Final", True, ['football', 'match', 'competition', 'sport']),
        create_event("UEFA Europa Conference League Final", True, ['football', 'match', 'competition', 'sport']),
        create_event("FIFA U-20 World Cup Final", True, ['football', 'match', 'competition', 'sport']),
        create_event("FIFA U-17 World Cup Final", True, ['football', 'match', 'competition', 'sport']),
    ]
}

events['basketball_matches'] = {
    'past': [
        create_event("NBA Finals Game 7 2024", False, ['basketball', 'match', 'competition', 'sport']),
        create_event("EuroLeague Championship", False, ['basketball', 'match', 'competition', 'sport']),
        create_event("WNBA Finals Game 5", False, ['basketball', 'match', 'competition', 'sport']),
        create_event("Olympic Basketball Final", False, ['basketball', 'match', 'competition', 'sport']),
        create_event("FIBA EuroBasket Final", False, ['basketball', 'match', 'competition', 'sport']),
        create_event("NCAA Championship Game", False, ['basketball', 'match', 'competition', 'sport']),
        create_event("NBA All-Star Game", False, ['basketball', 'match', 'competition', 'sport']),
    ],
    'future': [
        create_event("FIBA World Cup 2027", True, ['basketball', 'match', 'competition', 'sport']),
        create_event("NCAA Final Four 2028", True, ['basketball', 'match', 'competition', 'sport']), # Added
        create_event("Olympic Basketball Qualifiers", True, ['basketball', 'match', 'competition', 'sport']),
        create_event("EuroLeague Final Four", True, ['basketball', 'match', 'competition', 'sport']),
        create_event("WNBA All-Star Game", True, ['basketball', 'match', 'competition', 'sport']),
        create_event("FIBA U19 World Cup Final", True, ['basketball', 'match', 'competition', 'sport']),
        create_event("FIBA U17 World Cup Final", True, ['basketball', 'match', 'competition', 'sport']),
    ]
}

# Tennis
events['tennis_matches'] = {
    'past': [
        create_event("Wimbledon Men's Final 2023", False, ['tennis', 'match', 'competition', 'sport']),
        create_event("French Open Women's Semi-final", False, ['tennis', 'match', 'competition', 'sport']),
    ],
    'future': [
        create_event("Australian Open 2029", True, ['tennis', 'match', 'competition', 'sport']),
        create_event("US Open Men's Final 2027", True, ['tennis', 'match', 'competition', 'sport']),
        create_event("ATP Finals 2028", True, ['tennis', 'match', 'competition', 'sport']),
        create_event("WTA Finals 2028", True, ['tennis', 'match', 'competition', 'sport']),
        create_event("Davis Cup Final 2028", True, ['tennis', 'match', 'competition', 'sport']),
        create_event("Fed Cup Final 2028", True, ['tennis', 'match', 'competition', 'sport']),
        create_event("Olympic Tennis Final 2028", True, ['tennis', 'match', 'competition', 'sport']),
        create_event("Laver Cup 2028", True, ['tennis', 'match', 'competition', 'sport']),
        create_event("Ryder Cup 2028", True, ['tennis', 'match', 'competition', 'sport']),
        create_event("Hopman Cup 2028", True, ['tennis', 'match', 'competition', 'sport']),
        create_event("ATP Next Gen Finals 2028", True, ['tennis', 'match', 'competition', 'sport']),
    ]
}

# Volleyball
events['volleyball_matches'] = {
    'past': [
        create_event("FIVB World Championship Final", False, ['volleyball', 'match', 'competition', 'sport']),
        create_event("NCAA Volleyball Championship", False, ['volleyball', 'match', 'competition', 'sport']),
        create_event("AVP Beach Volleyball Final", False, ['volleyball', 'match', 'competition', 'sport']),
        create_event("CEV Champions League Final", False, ['volleyball', 'match', 'competition', 'sport']),
        create_event("FIVB Grand Prix Final", False, ['volleyball', 'match', 'competition', 'sport']),
        create_event("FIVB Club World Championship Final", False, ['volleyball', 'match', 'competition', 'sport']),
    ],
    'future': [
        create_event("Olympic Volleyball Tournament", True, ['volleyball', 'match', 'competition', 'sport']),
        create_event("FIVB World Championship 2026", True, ['volleyball', 'match', 'competition', 'sport']),
        create_event("NCAA Volleyball Championship 2027", True, ['volleyball', 'match', 'competition', 'sport']),
        create_event("AVP Beach Volleyball Tour Finals", True, ['volleyball', 'match', 'competition', 'sport']),
        create_event("CEV Champions League Final 2027", True, ['volleyball', 'match', 'competition', 'sport']),
        create_event("FIVB Grand Prix 2027", True, ['volleyball', 'match', 'competition', 'sport']),
        create_event("FIVB Club World Championship 2027", True, ['volleyball', 'match', 'competition', 'sport']),
        create_event("FIVB Nations League Final", True, ['volleyball', 'match', 'competition', 'sport']),
        create_event("FIVB U21 World Championship Final", True, ['volleyball', 'match', 'competition', 'sport']),
    ]
}

events['swimming_matches'] = {
    'past': [
        create_event("Olympic Swimming Finals 2024", False, ['swimming', 'competition', 'sport']),
        create_event("World Swimming Championships 2023", False, ['swimming', 'competition', 'sport']),
        create_event("European Swimming Championships 2023", False, ['swimming', 'competition', 'sport']),
        create_event("FINA World Cup 2023", False, ['swimming', 'competition', 'sport']),
        create_event("FINA Grand Prix 2023", False, ['swimming', 'competition', 'sport']),
        create_event("FINA Swimming World Series 2023", False, ['swimming', 'competition', 'sport']),
    ],
    'future': [
        create_event("Olympic Swimming Trials 2028", True, ['swimming', 'competition', 'sport']),
        create_event("World Swimming Championships 2028", True, ['swimming', 'competition', 'sport']),
        create_event("European Swimming Championships 2028", True, ['swimming', 'competition', 'sport']),
        create_event("FINA World Cup 2028", True, ['swimming', 'competition', 'sport']),
        create_event("FINA Grand Prix 2028", True, ['swimming', 'competition', 'sport']),
        create_event("FINA Swimming World Series 2028", True, ['swimming', 'competition', 'sport']),
        create_event("FINA Swimming World Cup 2028", True, ['swimming', 'competition', 'sport']),
        create_event("FINA Swimming World Championships 2028", True, ['swimming', 'competition', 'sport']),
    ]
}

events['cycling_matches'] = {
    'past': [
        create_event("Tour de France Stage 10", False, ['cycling', 'competition', 'sport']),
        create_event("Giro d'Italia Stage 5", False, ['cycling', 'competition', 'sport']),
        create_event("Vuelta a España Stage 8", False, ['cycling', 'competition', 'sport']),
        create_event("UCI Road World Championships", False, ['cycling', 'competition', 'sport']),
        create_event("Paris-Roubaix 2023", False, ['cycling', 'competition', 'sport']),
        create_event("Tour of Flanders 2023", False, ['cycling', 'competition', 'sport']),
        create_event("UCI Cyclo-cross World Championships", False, ['cycling', 'competition', 'sport']),
        create_event("UCI Mountain Bike World Championships", False, ['cycling', 'competition', 'sport']),
    ],
    'future': [
        create_event("Tour de France Stage 15", True, ['cycling', 'competition', 'sport']),
        create_event("Giro d'Italia Stage 10", True, ['cycling', 'competition', 'sport']),
        create_event("Vuelta a España Stage 12", True, ['cycling', 'competition', 'sport']),
        create_event("UCI Road World Championships 2028", True, ['cycling', 'competition', 'sport']),
        create_event("Paris-Roubaix 2028", True, ['cycling', 'competition', 'sport']),
        create_event("Tour of Flanders 2028", True, ['cycling', 'competition', 'sport']),
        create_event("UCI Cyclo-cross World Championships 2028", True, ['cycling', 'competition', 'sport']),
        create_event("UCI Mountain Bike World Championships 2028", True, ['cycling', 'competition', 'sport']),
        create_event("UCI BMX World Championships 2028", True, ['cycling', 'competition', 'sport']),
        create_event("UCI Track Cycling World Championships 2028", True, ['cycling', 'competition', 'sport']),
    ]
}

events['hiking_matches'] = {
    'past': [
        create_event("Guided Hike in the Tatra Mountains", False, ['hiking', 'social', 'outdoor', 'sport']),
        create_event("Guided Hike in the Bieszczady Mountains", False, ['hiking', 'social', 'outdoor', 'sport']),
        create_event("Guided Hike in the Pieniny Mountains", False, ['hiking', 'social', 'outdoor', 'sport']),
        create_event("Guided Hike in the Karkonosze Mountains", False, ['hiking', 'social', 'outdoor', 'sport']),
        create_event("Guided Hike in the Sudetes Mountains", False, ['hiking', 'social', 'outdoor', 'sport']),
        create_event("Guided Hike in the Beskidy Mountains", False, ['hiking', 'social', 'outdoor', 'sport']),
    ],
    'future': [
        create_event("Guided Hike in the Tatra Mountains", True, ['hiking', 'social', 'outdoor', 'sport']),
        create_event("Guided Hike in the Bieszczady Mountains", True, ['hiking', 'social', 'outdoor', 'sport']),
        create_event("Guided Hike in the Pieniny Mountains", True, ['hiking', 'social', 'outdoor', 'sport']),
        create_event("Guided Hike in the Karkonosze Mountains", True, ['hiking', 'social', 'outdoor', 'sport']),
        create_event("Guided Hike in the Sudetes Mountains", True, ['hiking', 'social', 'outdoor', 'sport']),
        create_event("Guided Hike in the Beskidy Mountains", True, ['hiking', 'social', 'outdoor', 'sport']),
        create_event("Guided Hike in the Tatra Mountains - Winter Edition", True, ['hiking', 'social', 'outdoor', 'sport']),
        create_event("Guided Hike in the Bieszczady Mountains - Winter Edition", True, ['hiking', 'social', 'outdoor', 'sport']),
    ]
}

events['running_matches'] = {
    'past': [
        create_event("Warsaw Marathon 2023", False, ['running', 'competition', 'marathon', 'sport']),
        create_event("Krakow Half Marathon 2023", False, ['running', 'competition', 'half_marathon', 'sport']),
        create_event("Gdynia 10K Run 2023", False, ['running', 'competition', '10k_run', 'sport']), 
        create_event("Poznan Marathon 2023", False, ['running', 'competition', 'marathon', 'sport']),
        create_event("Wroclaw Half Marathon 2023", False, ['running', 'competition', 'half_marathon', 'sport']),
        create_event("Lodz 10K Run 2023", False, ['running', 'competition', '10k_run', 'sport']),
    ],
    'future': [
        create_event("Warsaw Marathon 2027", True, ['running', 'competition', 'marathon', 'sport']),
        create_event("Krakow Half Marathon 2027", True, ['running', 'competition', 'half_marathon', 'sport']),
        create_event("Gdynia 10K Run 2027", True, ['running', 'competition', '10k_run', 'sport']),
        create_event("Poznan Marathon 2027", True, ['running', 'competition', 'marathon', 'sport']),
        create_event("Wroclaw Half Marathon 2027", True, ['running', 'competition', 'half_marathon', 'sport']),
        create_event("Lodz 10K Run 2027", True, ['running', 'competition', '10k_run', 'sport']),
        create_event("Warsaw Half Marathon 2027", True, ['running', 'competition', 'half_marathon', 'sport']),
        create_event("Krakow 10K Run 2027", True, ['running', 'competition', '10k_run', 'sport']),
        create_event("Gdynia Half Marathon 2027", True, ['running', 'competition', 'half_marathon', 'sport']),
        create_event("Poznan 10K Run 2027", True, ['running', 'competition', '10k_run', 'sport']),
        create_event("Wroclaw 10K Run 2027", True, ['running', 'competition', '10k_run', 'sport']),
        create_event("Lodz Half Marathon 2027", True, ['running', 'competition', 'half_marathon', 'sport']),
    ]
}

events['chess_matches'] = {
    'past': [
        create_event("World Chess Championship 2023", False, ['chess', 'match', 'competition']),
        create_event("Candidates Tournament 2023", False, ['chess', 'match', 'competition']),
        create_event("Grand Slam Chess Tournament 2023", False, ['chess', 'match', 'competition']),
        create_event("Chess Olympiad 2023", False, ['chess', 'match', 'competition']),
        create_event("World Rapid Chess Championship 2023", False, ['chess', 'match', 'competition']),
        create_event("World Blitz Chess Championship 2023", False, ['chess', 'match', 'competition']),
        create_event("FIDE Grand Prix 2023", False, ['chess', 'match', 'competition']),
    ],
    'future': [
        create_event("World Chess Championship 2028", True, ['chess', 'match', 'competition']),
        create_event("Candidates Tournament 2028", True, ['chess', 'match', 'competition']),
        create_event("Grand Slam Chess Tournament 2028", True, ['chess', 'match', 'competition']),
        create_event("Chess Olympiad 2028", True, ['chess', 'match', 'competition']),
        create_event("World Rapid Chess Championship 2028", True, ['chess', 'match', 'competition']),
        create_event("World Blitz Chess Championship 2028", True, ['chess', 'match', 'competition']),
        create_event("FIDE Grand Prix 2028", True, ['chess', 'match', 'competition']),
    ]
}

events['card_games'] = {
    'past': [
        create_event("World Series of Poker 2023", False, ['card_games', 'poker', 'tournament']),
        create_event("Magic: The Gathering Pro Tour 2023", False, ['card_games', 'tcg', 'tournament']),
        create_event("Yu-Gi-Oh! World Championship 2023", False, ['card_games', 'tcg', 'tournament']),
        create_event("Hearthstone World Championship 2023", False, ['card_games', 'online', 'tournament']),
        create_event("Gwent Masters 2023", False, ['card_games', 'online', 'tournament']),
        create_event("Flesh and Blood World Championship 2023", False, ['card_games', 'tcg', 'tournament']),
        create_event("KeyForge World Championship 2023", False, ['card_games', 'tcg', 'tournament']),
        create_event("Pokemon World Championship 2023", False, ['card_games', 'tcg', 'tournament']),
        create_event("Magic: The Gathering World Championship 2023", False, ['card_games', 'tcg', 'tournament']),
        create_event("World Championship of Online Poker 2023", False, ['card_games', 'online', 'tournament']),
        create_event("World Championship of Online Poker 2023", False, ['card_games', 'online', 'tournament']),
        create_event("World Championship of Online Poker 2023", False, ['card_games', 'online', 'tournament']),
        create_event("World Championship of Online Poker 2023", False, ['card_games', 'online', 'tournament']),
    ],
    'future': [
        create_event("World Series of Poker 2028", True, ['card_games', 'poker', 'tournament']),
        create_event("Magic: The Gathering Pro Tour 2028", True, ['card_games', 'tcg', 'tournament']),
        create_event("Yu-Gi-Oh! World Championship 2028", True, ['card_games', 'tcg', 'tournament']),
        create_event("Hearthstone World Championship 2028", True, ['card_games', 'online', 'tournament']),
        create_event("Gwent Masters 2028", True, ['card_games', 'online', 'tournament']),
        create_event("Flesh and Blood World Championship 2028", True, ['card_games', 'tcg', 'tournament']),
        create_event("KeyForge World Championship 2028", True, ['card_games', 'tcg', 'tournament']),
        create_event("Pokemon World Championship 2028", True, ['card_games', 'tcg', 'tournament']),
        create_event("Magic: The Gathering World Championship 2028", True, ['card_games', 'tcg', 'tournament']),
        create_event("World Championship of Online Poker 2028", True, ['card_games', 'online', 'tournament']),
    ]
}

# Dance Classes
events['dance_classes'] = {
    'past': [
        create_event("Beginner Hip Hop Class", False, ['hip_hop_dance', 'class', 'dance']),
        create_event("Intermediate Ballet Workshop", False, ['ballet', 'workshop', 'dance']),
        create_event("Salsa Social Dance Night", False, ['salsa', 'social', 'dance']),
        create_event("Contemporary Dance Performance", False, ['contemporary_dance', 'live', 'performance', 'dance']),
        create_event("Ballroom Dance Competition", False, ['ballroom_dance', 'competition', 'dance']),
        create_event("Jazz Dance Workshop", False, ['jazz_dance', 'workshop', 'dance']),
        create_event("Ballet Recital", False, ['ballet', 'live', 'performance', 'dance']),
        create_event("Hip Hop Dance Battle", False, ['hip_hop_dance', 'competition', 'dance']),
        create_event("Salsa Dance Festival", False, ['salsa', 'festival', 'dance']),
        create_event("Zumba Dance Party", False, ['zumba', 'social', 'dance']),
        create_event("Contemporary Dance Workshop", False, ['contemporary_dance', 'workshop', 'dance']),
        create_event("Ballroom Dance Showcase", False, ['ballroom_dance', 'live', 'performance', 'dance']),
        create_event("Jazz Dance Performance", False, ['jazz_dance', 'live', 'performance', 'dance']),
    ],
    'future': [
        create_event("Zumba Fitness Session", True, ['zumba', 'class', 'dance']),
        create_event("Advanced Salsa Class Series", True, ['salsa', 'class', 'dance']),
        create_event("Contemporary Dance Performance", True, ['contemporary_dance', 'live', 'performance', 'dance']),
        create_event("Ballroom Dance Competition", True, ['ballroom_dance', 'competition', 'dance']),
        create_event("Jazz Dance Workshop", True, ['jazz_dance', 'workshop', 'dance']),
        create_event("Ballet Recital", True, ['ballet', 'live', 'performance', 'dance']),
        create_event("Hip Hop Dance Battle", True, ['hip_hop_dance', 'competition', 'dance']),
        create_event("Salsa Dance Festival", True, ['salsa', 'festival', 'dance']),
        create_event("Zumba Dance Party", True, ['zumba', 'social', 'dance']),
        create_event("Contemporary Dance Workshop", True, ['contemporary_dance', 'workshop', 'dance']),
        create_event("Ballroom Dance Showcase", True, ['ballroom_dance', 'live', 'performance', 'dance']),
        create_event("Jazz Dance Performance", True, ['jazz_dance', 'live', 'performance', 'dance']),
        create_event("Ballet Dance Class", True, ['ballet', 'class', 'dance']),
        create_event("Hip Hop Dance Class", True, ['hip_hop_dance', 'class', 'dance']),
        create_event("Salsa Dance Class", True, ['salsa', 'class', 'dance']),
        create_event("Zumba Dance Class", True, ['zumba', 'class', 'dance']),
        create_event("Contemporary Dance Class", True, ['contemporary_dance', 'class', 'dance']),
        create_event("Ballroom Dance Class", True, ['ballroom_dance', 'class', 'dance']),
        create_event("Jazz Dance Class", True, ['jazz_dance', 'class', 'dance']),
        create_event("Ballet Dance Workshop", True, ['ballet', 'workshop', 'dance']),
        create_event("Hip Hop Dance Workshop", True, ['hip_hop_dance', 'workshop', 'dance']),
        create_event("Salsa Dance Workshop", True, ['salsa', 'workshop', 'dance']),
    ]
}

events['yoga_classes'] = {
    'past': [
        create_event("Morning Vinyasa Flow", False, ['yoga', 'class']),
        create_event("Yoga for Beginners Workshop", False, ['yoga', 'workshop']),
        create_event("Yoga for Athletes Workshop", False, ['yoga', 'workshop']),
        create_event("Yoga for Stress Relief Workshop", False, ['yoga', 'workshop']),
        create_event("Yoga for Flexibility Workshop", False, ['yoga', 'workshop']),
        create_event("Yoga for Strength Workshop", False, ['yoga', 'workshop']),
        create_event("Yoga for Balance Workshop", False, ['yoga', 'workshop']),
        create_event("Yoga for Mindfulness Workshop", False, ['yoga', 'workshop']),
    ],
    'future': [
        create_event("Evening Restorative Yoga", True, ['yoga', 'class']),
        create_event("Yoga Teacher Training Info Session", True, ['yoga', 'workshop']),
        create_event("Yoga for Athletes Workshop", True, ['yoga', 'workshop']),
        create_event("Yoga for Stress Relief Workshop", True, ['yoga', 'workshop']),
        create_event("Yoga for Flexibility Workshop", True, ['yoga', 'workshop']),
        create_event("Yoga for Strength Workshop", True, ['yoga', 'workshop']),
        create_event("Yoga for Balance Workshop", True, ['yoga', 'workshop']),
        create_event("Yoga for Mindfulness Workshop", True, ['yoga', 'workshop']),
        create_event("Yoga for Seniors Workshop", True, ['yoga', 'workshop']),
        create_event("Yoga for Kids Workshop", True, ['yoga', 'workshop']),
    ]
}

events['computer_science_conferences'] = {
    'past': [
        create_event("AI and Machine Learning Conference 2023", False, ['technology', 'conference']),
        create_event("Blockchain Technology Summit 2023", False, ['technology', 'conference']),
        create_event("Cybersecurity and Data Protection Conference 2023", False, ['technology', 'conference']),
        create_event("Cloud Computing and Virtualization Conference 2023", False, ['technology', 'conference']),
        create_event("Big Data and Analytics Conference 2023", False, ['technology', 'conference']),
        create_event("Internet of Things (IoT) Conference 2023", False, ['technology', 'conference']),
        create_event("Augmented Reality and Virtual Reality Conference 2023", False, ['technology', 'conference']),
        create_event("5G Technology Conference 2023", False, ['technology', 'conference']),
        create_event("Quantum Computing Conference 2023", False, ['technology', 'conference']),
    ],
    'future': [
        create_event("AI and Machine Learning Conference 2028", True, ['technology', 'conference']),
        create_event("Blockchain Technology Summit 2028", True, ['technology', 'conference']),
        create_event("Cybersecurity and Data Protection Conference 2028", True, ['technology', 'conference']),
        create_event("Cloud Computing and Virtualization Conference 2028", True, ['technology', 'conference']),
        create_event("Big Data and Analytics Conference 2028", True, ['technology', 'conference']),
        create_event("Internet of Things (IoT) Conference 2028", True, ['technology', 'conference']),
        create_event("Augmented Reality and Virtual Reality Conference 2028", True, ['technology', 'conference']),
        create_event("5G Technology Conference 2028", True, ['technology', 'conference']),
        create_event("Quantum Computing Conference 2028", True, ['technology', 'conference']),
        create_event("Future of AI Conference 2028", True, ['technology', 'conference']), # Added
        create_event("Future of Quantum Computing Conference 2028", True, ['technology', 'conference']), # Added
    ]
}

events['photography_walks'] = {
    'past': [
        create_event("Nature Photography Walk", False, ['photography', 'social', 'outdoor']),
        create_event("Urban Photography Walk", False, ['photography', 'social', 'outdoor']),
        create_event("Street Photography Walk", False, ['photography', 'social', 'outdoor']),
        create_event("Wildlife Photography Walk", False, ['photography', 'social', 'outdoor']),
    ],
    'future': [
        create_event("Nature Photography Walk", True, ['photography', 'social', 'outdoor']),
        create_event("Urban Photography Walk", True, ['photography', 'social', 'outdoor']),
        create_event("Street Photography Walk", True, ['photography', 'social', 'outdoor']),
        create_event("Wildlife Photography Walk", True, ['photography', 'social', 'outdoor']),
        create_event("Portrait Photography Walk", True, ['photography', 'social', 'outdoor']),
        create_event("Landscape Photography Walk", True, ['photography', 'social', 'outdoor']),
        create_event("Astrophotography Walk", True, ['photography', 'social', 'outdoor']),
    ]
}

events['cooking_classes'] = {
    'past': [
        create_event("Italian Cuisine Cooking Class", False, ['cooking', 'workshop']),
        create_event("French Cuisine Cooking Class", False, ['cooking', 'workshop']),
        create_event("Japanese Cuisine Cooking Class", False, ['cooking', 'workshop']),
        create_event("Mexican Cuisine Cooking Class", False, ['cooking', 'workshop']),
        create_event("Indian Cuisine Cooking Class", False, ['cooking', 'workshop']),
        create_event("Mediterranean Cuisine Cooking Class", False, ['cooking', 'workshop']),
        create_event("Baking Basics Cooking Class", False, ['cooking', 'workshop']),
        create_event("Vegetarian Cooking Class", False, ['cooking', 'workshop']),
        create_event("Vegan Cooking Class", False, ['cooking', 'workshop']),
        create_event("Gluten-Free Cooking Class", False, ['cooking', 'workshop']),
    ],
    'future': [
        create_event("Italian Cuisine Cooking Class", True, ['cooking', 'workshop']),
        create_event("French Cuisine Cooking Class", True, ['cooking', 'workshop']),
        create_event("Japanese Cuisine Cooking Class", True, ['cooking', 'workshop']),
        create_event("Mexican Cuisine Cooking Class", True, ['cooking', 'workshop']),
        create_event("Indian Cuisine Cooking Class", True, ['cooking', 'workshop']),
        create_event("Mediterranean Cuisine Cooking Class", True, ['cooking', 'workshop']),
        create_event("Baking Basics Cooking Class", True, ['cooking', 'workshop']),
        create_event("Vegetarian Cooking Class", True, ['cooking', 'workshop']),
        create_event("Vegan Cooking Class", True, ['cooking', 'workshop']),
        create_event("Gluten-Free Cooking Class", True, ['cooking', 'workshop']),
        create_event("Healthy Cooking Class", True, ['cooking', 'workshop']),
    ]
}

events['writing_groups'] = {
    'past': [
        create_event("Creative Writing Workshop", False, ['writing', 'workshop']),
        create_event("Poetry Writing Workshop", False, ['writing', 'workshop']),
        create_event("Fiction Writing Workshop", False, ['writing', 'workshop']),
        create_event("Non-Fiction Writing Workshop", False, ['writing', 'workshop']),
        create_event("Memoir Writing Workshop", False, ['writing', 'workshop']),
    ],
    'future': [
        create_event("Creative Writing Workshop", True, ['writing', 'workshop']),
        create_event("Poetry Writing Workshop", True, ['writing', 'workshop']),
        create_event("Fiction Writing Workshop", True, ['writing', 'workshop']),
        create_event("Non-Fiction Writing Workshop", True, ['writing', 'workshop']),
        create_event("Memoir Writing Workshop", True, ['writing', 'workshop']),
        create_event("Screenwriting Workshop", True, ['writing', 'workshop']),
        create_event("Playwriting Workshop", True, ['writing', 'workshop']),
    ]
}

events['science_conferences'] = {
    'past': [
        create_event("Climate Change Conference 2023", False, ['science', 'conference']),
        create_event("Genetics and Genomics Conference 2023", False, ['science', 'conference']),
        create_event("Neuroscience Conference 2023", False, ['science', 'conference']),
        create_event("Physics and Astronomy Conference 2023", False, ['science', 'conference']),
        create_event("Chemistry Conference 2023", False, ['science', 'conference']),
        create_event("Biology Conference 2023", False, ['science', 'conference']),
        create_event("Environmental Science Conference 2023", False, ['science', 'conference']),
    ],
    'future': [
        create_event("Climate Change Conference 2028", True, ['science', 'conference']),
        create_event("Genetics and Genomics Conference 2028", True, ['science', 'conference']),
        create_event("Neuroscience Conference 2028", True, ['science', 'conference']),
        create_event("Physics and Astronomy Conference 2028", True, ['science', 'conference']),
        create_event("Chemistry Conference 2028", True, ['science', 'conference']),
    ]
}
