# Rust Dojo — Duolingo-style Rust lessons

A tiny, dependency-free static website that turns the tutorials in this repository
into interactive lessons: a path of units, XP, streaks, hearts, and per-exercise
feedback.

All **lesson content is data-driven** from [`data/lessons.json`](data/lessons.json),
and **your progress can be exported/imported as JSON**.

## Files

```
site/
├── index.html        # markup shell (header, stats, #app container)
├── styles.css        # Duolingo-ish styling, no framework
├── app.js            # loads lessons.json, runs the game, saves progress
├── data/
│   └── lessons.json  # ALL lesson/unit/exercise content (edit me!)
└── README.md
```

## Running it

Browsers block `fetch()` on `file://` pages, so serve the folder over HTTP.

```bash
cd site
python3 -m http.server 8000
# then open http://localhost:8000
```

Any static server works (e.g. `npx serve`, `php -S localhost:8000`).

## How the game works

- **Path** — lessons are grouped into units. The first lesson is unlocked; completing a
  lesson unlocks the next one in the sequence.
- **Exercises** — three types: `multiple-choice`, `true-false`, and `fill-blank`.
  After answering you get feedback with an explanation.
- **Hearts** — start a lesson with 5 hearts; each wrong answer costs one. At zero you
  must retry the lesson.
- **XP & streak** — finishing a lesson awards XP and updates a daily streak.
- **Progress** is saved in your browser's `localStorage` (key `rust-dojo-progress-v1`).

## Export / import JSON

- **Export** (header button) downloads `rust-dojo-progress.json` containing your
  `progress` **and** the current `lessons` catalog.
- **Import** reads that file back and restores your progress.
- **Reset** clears stored progress.

## Editing the lessons

Everything lives in `data/lessons.json`. The shape is:

```jsonc
{
  "title": "Rust Dojo",
  "version": 1,
  "units": [
    {
      "id": "unit-1",
      "title": "Unit 1 - Getting Started",
      "description": "Output, variables, and the basic types.",
      "color": "#58cc02",          // unit banner colour
      "lessons": [
        {
          "id": "lesson-1",        // must be unique; used to track completion
          "title": "Hello, Rust!",
          "xp": 10,                // XP awarded on first completion
          "exercises": [
            /* see below */
          ]
        }
      ]
    }
  ]
}
```

### Exercise types

**multiple-choice** — one correct `answer` (0-based index into `options`):

```json
{
  "type": "multiple-choice",
  "prompt": "What does this print?",
  "code": "println!(\"Hi\");",
  "options": ["Hi", "hi", "Nothing", "Error"],
  "answer": 0,
  "explanation": "println! prints its argument and a newline."
}
```

**true-false** — `answer` is a boolean:

```json
{
  "type": "true-false",
  "prompt": "Conditions must be bool.",
  "answer": true,
  "explanation": "Rust has no truthy/falsy coercion."
}
```

**fill-blank** — user types an answer; any string in `answers` is accepted
(matching is case-insensitive and whitespace-normalised):

```json
{
  "type": "fill-blank",
  "prompt": "Which keyword makes a binding mutable?",
  "template": "let ___ x = 4;",
  "answers": ["mut"],
  "explanation": "mut makes the binding reassignable."
}
```

Both `code` and `template` are optional and render in a code block.

## Adding a lesson

1. Add a new object to a unit's `lessons` array with a unique `id`.
2. Add exercises to it.
3. Reload the page. Unlocking is positional, so a new lesson is available once the one
   before it is completed.
