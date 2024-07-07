Project Structure:

src/
├── main.rs
├── lib.rs
├── core/
│   ├── mod.rs
│   ├── journal.rs
│   └── entry.rs
├── api/
│   ├── mod.rs
│   └── journal_api.rs
├── ui/
│   ├── mod.rs
│   ├── cli.rs
│   └── display.rs
└── utils/
    ├── mod.rs
    └── io.rs

Cargo.toml

Pseudo-code:

// main.rs
FUNCTION main():
    CREATE new CLI instance
    RUN CLI instance
    HANDLE any errors

// core/journal.rs
CLASS Journal:
    PROPERTIES:
        entries: List of Entry
        next_id: Integer

    METHODS:
        add_entry(title, content):
            CREATE new Entry with next_id, title, content
            ADD Entry to entries list
            INCREMENT next_id
            RETURN new Entry's id

        get_entries():
            RETURN list of all entries

        get_entry(id):
            FIND and RETURN entry with matching id
            IF not found, RETURN None

        update_entry(id, title, content):
            FIND entry with matching id
            IF found:
                UPDATE entry's title and content
                RETURN True
            ELSE:
                RETURN False

        delete_entry(id):
            REMOVE entry with matching id from entries list
            RETURN True if entry was found and removed, False otherwise

// core/entry.rs
CLASS Entry:
    PROPERTIES:
        id: Integer
        title: String
        content: String
        created_at: DateTime

    METHODS:
        constructor(id, title, content):
            SET properties
            SET created_at to current date and time

// api/journal_api.rs
CLASS JournalApi:
    PROPERTIES:
        journal: Journal instance

    METHODS:
        add_entry(title, content):
            VALIDATE title is not empty
            CALL journal.add_entry(title, content)
            RETURN Result with new entry id or error

        get_entries():
            GET all entries from journal
            TRANSFORM entries to EntryView objects
            RETURN list of EntryView objects

        get_entry(id):
            GET entry from journal
            IF entry found:
                TRANSFORM to EntryDetail object
                RETURN Result with EntryDetail
            ELSE:
                RETURN Result with error

        update_entry(id, title, content):
            VALIDATE title is not empty
            CALL journal.update_entry(id, title, content)
            RETURN Result with success or error

        delete_entry(id):
            CALL journal.delete_entry(id)
            RETURN Result with success or error

// ui/cli.rs
CLASS Cli:
    PROPERTIES:
        api: JournalApi instance

    METHODS:
        run():
            LOOP:
                DISPLAY menu
                GET user choice
                MATCH choice:
                    WHEN add entry: CALL add_entry()
                    WHEN list entries: CALL list_entries()
                    WHEN view entry: CALL view_entry()
                    WHEN update entry: CALL update_entry()
                    WHEN delete entry: CALL delete_entry()
                    WHEN exit: BREAK loop
                    OTHERWISE: DISPLAY error message

        add_entry():
            GET title from user
            GET content from user
            CALL api.add_entry(title, content)
            DISPLAY result (success or error)

        list_entries():
            GET entries from api
            DISPLAY entries

        view_entry():
            GET entry id from user
            CALL api.get_entry(id)
            IF entry found:
                DISPLAY entry details
            ELSE:
                DISPLAY error message

        update_entry():
            GET entry id from user
            GET new title from user
            GET new content from user
            CALL api.update_entry(id, title, content)
            DISPLAY result (success or error)

        delete_entry():
            GET entry id from user
            CALL api.delete_entry(id)
            DISPLAY result (success or error)

// ui/display.rs
CLASS Display:
    METHODS:
        show_menu():
            PRINT menu options

        show_entry_added(id):
            PRINT success message with entry id

        show_entry_list(entries):
            FOR EACH entry in entries:
                PRINT entry summary (id, title, date)

        show_entry_detail(entry):
            PRINT entry details (title, content, date)

        show_error(message):
            PRINT error message

// utils/io.rs
FUNCTION get_user_input(prompt):
    PRINT prompt
    READ user input
    RETURN trimmed input