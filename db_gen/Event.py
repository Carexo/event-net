class Event:
    def __init__(self, id, name, startDatetime):
        self.id = id
        self.name = name
        self.startDatetime = startDatetime
        self.keywords = [] # List of EventKeyword objects

    def add_keyword(self, keyword):
        self.keywords.append(keyword)