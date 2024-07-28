### SQLite Query Dependency

You can create a step dependency with an SQLite query. When the query results
change, the step is invalidated.

SQLite dependencies doesn't track the results of the query. It just checks
whether the query results has changed.

This command works only in Xvc repositories.

```console
$ git init
...
$ xvc init
```

Suppose we have an SQLite database `people.db` with the following schema and data:

```sql
CREATE TABLE People (
    Name TEXT,
    Sex TEXT,
    Age INTEGER,
    Height_in INTEGER,
    Weight_lbs INTEGER
);

INSERT INTO People (Name, Sex, Age, Height_in, Weight_lbs) VALUES
('Alex', 'M', 41, 74, 170),
('Bert', 'M', 42, 68, 166),
('Carl', 'M', 32, 70, 155),
('Dave', 'M', 39, 72, 167),
('Elly', 'F', 30, 66, 124),
('Fran', 'F', 33, 66, 115),
('Gwen', 'F', 26, 64, 121),
('Hank', 'M', 30, 71, 158),
('Ivan', 'M', 53, 72, 175),
('Jake', 'M', 32, 69, 143),
('Kate', 'F', 47, 69, 139),
('Luke', 'M', 34, 72, 163),
('Myra', 'F', 23, 62, 98),
('Neil', 'M', 36, 75, 160),
('Omar', 'M', 38, 70, 145),
('Page', 'F', 31, 67, 135),
('Quin', 'M', 29, 71, 176),
('Ruth', 'F', 28, 65, 131);
EOF

```

Now, we'll add a step to the pipeline to calculate the average age of these people.

```console
$ xvc pipeline step new --step-name average-age --command "sqlite3 people.db 'SELECT AVG(Age) FROM People;'"

```

Let's run the step without a dependency first.

```console
$ xvc pipeline run
[OUT] [average-age] 34.6666666666667

[DONE] average-age (sqlite3 people.db 'SELECT AVG(Age) FROM People;')


```

Now, we'll add a dependency to this step and it will only run the step when the results of that query changes.

```console
$ xvc pipeline step dependency --step-name average-age --sqlite-query people.db 'SELECT count(*) FROM People;'

```

```admonition note
The dependency query is run everytime the pipeline runs. It's expected to be lightweight to avoid performance issues.
```

So, when the number of people in the table changes, the step will run. Initially it doesn't keep track of the query results, so it will run again.

```console
$ xvc pipeline run
[DEBUG][logging/src/lib.rs::237] Terminal logger enabled with level: Trace
[TRACE][core/src/types/xvcroot.rs::263] xvc_candidate: "[CWD]/.xvc"
[TRACE][core/src/types/xvcroot.rs::265] parent: "[CWD]"
[DEBUG][config/src/error.rs::72] Config source for level "system" not found at "/Users/iex/Library/Application Support/com.emresult.xvc"
[DEBUG][config/src/error.rs::72] Config source for level "global" not found at "/Users/iex/Library/Application Support/xvc"
[TRACE][ecs/src/ecs/mod.rs::229] dir: "[CWD]/.xvc/ec"
[TRACE][ecs/src/ecs/mod.rs::239] files: [
    "[CWD]/.xvc/ec/1722171155879620",
    "[CWD]/.xvc/ec/1722171155882390",
    "[CWD]/.xvc/ec/1722171155967050",
    "[CWD]/.xvc/ec/1722171156186854",
]
[TRACE][core/src/types/xvcroot.rs::263] xvc_candidate: "[CWD]/.xvc"
[TRACE][core/src/types/xvcroot.rs::265] parent: "[CWD]"
[TRACE][pipeline/src/pipeline/mod.rs::285] pipeline_e: XvcEntity(
    1,
    1872241824668543094,
)
[TRACE][pipeline/src/pipeline/mod.rs::290] pipeline_steps: HStore {
    map: {
        XvcEntity(
            2,
            9270302598983016314,
        ): XvcStep {
            name: "average-age",
        },
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::293] consider_changed: XvcStore {
    map: {
        XvcEntity(
            2,
            9270302598983016314,
        ): ByDependencies,
    },
    entity_index: {
        ByDependencies: [
            XvcEntity(
                2,
                9270302598983016314,
            ),
        ],
    },
    previous: EventLog(
        [
            Add {
                entity: XvcEntity(
                    2,
                    9270302598983016314,
                ),
                value: ByDependencies,
            },
        ],
    ),
    current: EventLog(
        [],
    ),
}
[TRACE][pipeline/src/pipeline/mod.rs::296] all_deps.parents.len(): 1
[TRACE][pipeline/src/pipeline/mod.rs::297] all_deps.children.len(): 1
[TRACE][pipeline/src/pipeline/mod.rs::299] all_outs.parents.len(): 1
[TRACE][pipeline/src/pipeline/mod.rs::300] all_outs.children.len(): 0
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.14/src/lib.rs::453] built glob set; 0 literals, 2 basenames, 0 extensions, 0 prefixes, 0 suffixes, 0 required extensions, 0 regexes
[TRACE][walker/src/lib.rs::488] ignore_fn: ".xvcignore"
[TRACE][walker/src/lib.rs::490] ignore_root: "[CWD]"
[TRACE][walker/src/lib.rs::498] entry.path(): "[CWD]/.xvc"
[TRACE][walker/src/lib.rs::498] entry.path(): "[CWD]/.git"
[TRACE][walker/src/lib.rs::503] ignore_path: "[CWD]/.xvcignore"
[TRACE][walker/src/lib.rs::597] ignore_root: "[CWD]"
[TRACE][walker/src/lib.rs::598] ignore_path: "[CWD]/.xvcignore"
[TRACE][walker/src/lib.rs::606] &content: "
# Add patterns of files xvc should ignore, which could improve
# the performance.
# It's in the same format as .gitignore files.

.DS_Store
"
[DEBUG][/Users/iex/.cargo/registry/src/index.crates.io-6f17d22bba15001f/globset-0.4.14/src/lib.rs::453] built glob set; 0 literals, 3 basenames, 0 extensions, 0 prefixes, 0 suffixes, 0 required extensions, 0 regexes
[TRACE][walker/src/lib.rs::748] is_abs: true
[TRACE][walker/src/lib.rs::752] path_str: "[CWD]/.xvc"
[TRACE][walker/src/lib.rs::754] final_slash: false
[TRACE][walker/src/lib.rs::776] path: "/.xvc"
[TRACE][walker/src/lib.rs::748] is_abs: true
[TRACE][walker/src/lib.rs::752] path_str: "[CWD]/.git"
[TRACE][walker/src/lib.rs::754] final_slash: false
[TRACE][walker/src/lib.rs::776] path: "/.git"
[TRACE][core/src/util/pmp.rs::41] ignore_rules: IgnoreRules {
    root: "[CWD]",
    ignore_patterns: RwLock {
        data: [
            Pattern {
                pattern: Glob {
                    glob: "**/.xvc",
                    re: "(?-u)^(?:/?|.*/)//.xvc$",
                    opts: GlobOptions {
                        case_insensitive: false,
                        literal_separator: false,
                        backslash_escape: true,
                        empty_alternates: false,
                    },
                    tokens: Tokens(
                        [
                            RecursivePrefix,
                            Literal(
                                '.',
                            ),
                            Literal(
                                'x',
                            ),
                            Literal(
                                'v',
                            ),
                            Literal(
                                'c',
                            ),
                        ],
                    ),
                },
                original: ".xvc",
                source: Global,
                effect: Ignore,
                relativity: Anywhere,
                path_kind: Any,
            },
            Pattern {
                pattern: Glob {
                    glob: "**/.git",
                    re: "(?-u)^(?:/?|.*/)//.git$",
                    opts: GlobOptions {
                        case_insensitive: false,
                        literal_separator: false,
                        backslash_escape: true,
                        empty_alternates: false,
                    },
                    tokens: Tokens(
                        [
                            RecursivePrefix,
                            Literal(
                                '.',
                            ),
                            Literal(
                                'g',
                            ),
                            Literal(
                                'i',
                            ),
                            Literal(
                                't',
                            ),
                        ],
                    ),
                },
                original: ".git",
                source: Global,
                effect: Ignore,
                relativity: Anywhere,
                path_kind: Any,
            },
            Pattern {
                pattern: Glob {
                    glob: "**/.DS_Store",
                    re: "(?-u)^(?:/?|.*/)//.DS_Store$",
                    opts: GlobOptions {
                        case_insensitive: false,
                        literal_separator: false,
                        backslash_escape: true,
                        empty_alternates: false,
                    },
                    tokens: Tokens(
                        [
                            RecursivePrefix,
                            Literal(
                                '.',
                            ),
                            Literal(
                                'D',
                            ),
                            Literal(
                                'S',
                            ),
                            Literal(
                                '_',
                            ),
                            Literal(
                                'S',
                            ),
                            Literal(
                                't',
                            ),
                            Literal(
                                'o',
                            ),
                            Literal(
                                'r',
                            ),
                            Literal(
                                'e',
                            ),
                        ],
                    ),
                },
                original: ".DS_Store",
                source: File {
                    path: ".xvcignore",
                    line: 6,
                },
                effect: Ignore,
                relativity: Anywhere,
                path_kind: Any,
            },
        ],
        poisoned: false,
        ..
    },
    whitelist_patterns: RwLock {
        data: [],
        poisoned: false,
        ..
    },
    whitelist_set: RwLock {
        data: GlobSet {
            len: 0,
            strats: [],
        },
        poisoned: false,
        ..
    },
    ignore_set: RwLock {
        data: GlobSet {
            len: 3,
            strats: [
                Extension(
                    ExtensionStrategy(
                        {},
                    ),
                ),
                BasenameLiteral(
                    BasenameLiteralStrategy(
                        {
                            [
                                46,
                                120,
                                118,
                                99,
                            ]: [
                                0,
                            ],
                            [
                                46,
                                68,
                                83,
                                95,
                                83,
                                116,
                                111,
                                114,
                                101,
                            ]: [
                                2,
                            ],
                            [
                                46,
                                103,
                                105,
                                116,
                            ]: [
                                1,
                            ],
                        },
                    ),
                ),
                Literal(
                    LiteralStrategy(
                        {},
                    ),
                ),
                Suffix(
                    SuffixStrategy {
                        matcher: AhoCorasick(
                            dfa::DFA(
                            D 000000: /x00 => 0
                            F 000001:
                             >000002: /x00 => 2
                              000003: /x00 => 0
                            match kind: Standard
                            prefilter: false
                            state length: 4
                            pattern length: 0
                            shortest pattern length: 18446744073709551615
                            longest pattern length: 0
                            alphabet length: 1
                            stride: 1
                            byte classes: ByteClasses(0 => [0-255])
                            memory usage: 16
                            )
                            ,
                        ),
                        map: [],
                        longest: 0,
                    },
                ),
                Prefix(
                    PrefixStrategy {
                        matcher: AhoCorasick(
                            dfa::DFA(
                            D 000000: /x00 => 0
                            F 000001:
                             >000002: /x00 => 2
                              000003: /x00 => 0
                            match kind: Standard
                            prefilter: false
                            state length: 4
                            pattern length: 0
                            shortest pattern length: 18446744073709551615
                            longest pattern length: 0
                            alphabet length: 1
                            stride: 1
                            byte classes: ByteClasses(0 => [0-255])
                            memory usage: 16
                            )
                            ,
                        ),
                        map: [],
                        longest: 0,
                    },
                ),
                RequiredExtension(
                    RequiredExtensionStrategy(
                        {},
                    ),
                ),
                Regex(
                    RegexSetStrategy {
                        matcher: Regex {
                            imp: RegexI {
                                strat: Core {
                                    info: RegexInfo(
                                        RegexInfoI {
                                            config: Config {
                                                match_kind: Some(
                                                    All,
                                                ),
                                                utf8_empty: Some(
                                                    false,
                                                ),
                                                autopre: None,
                                                pre: None,
                                                which_captures: None,
                                                nfa_size_limit: Some(
                                                    Some(
                                                        10485760,
                                                    ),
                                                ),
                                                onepass_size_limit: None,
                                                hybrid_cache_capacity: Some(
                                                    10485760,
                                                ),
                                                hybrid: None,
                                                dfa: None,
                                                dfa_size_limit: None,
                                                dfa_state_limit: None,
                                                onepass: None,
                                                backtrack: None,
                                                byte_classes: None,
                                                line_terminator: None,
                                            },
                                            props: [],
                                            props_union: Properties(
                                                PropertiesI {
                                                    minimum_len: None,
                                                    maximum_len: None,
                                                    look_set: ∅,
                                                    look_set_prefix: ∅,
                                                    look_set_suffix: ∅,
                                                    look_set_prefix_any: ∅,
                                                    look_set_suffix_any: ∅,
                                                    utf8: true,
                                                    explicit_captures_len: 0,
                                                    static_explicit_captures_len: None,
                                                    literal: false,
                                                    alternation_literal: true,
                                                },
                                            ),
                                        },
                                    ),
                                    pre: None,
                                    nfa: thompson::NFA(
                                    ^000000: FAIL

                                    transition equivalence classes: ByteClasses(0 => [/x00-/xFF], 1 => [EOI])
                                    )
                                    ,
                                    nfarev: Some(
                                        thompson::NFA(
                                        ^000000: FAIL

                                        transition equivalence classes: ByteClasses(0 => [/x00-/xFF], 1 => [EOI])
                                        )
                                        ,
                                    ),
                                    pikevm: PikeVM(
                                        PikeVMEngine(
                                            PikeVM {
                                                config: Config {
                                                    match_kind: Some(
                                                        All,
                                                    ),
                                                    pre: Some(
                                                        None,
                                                    ),
                                                },
                                                nfa: thompson::NFA(
                                                ^000000: FAIL

                                                transition equivalence classes: ByteClasses(0 => [/x00-/xFF], 1 => [EOI])
                                                )
                                                ,
                                            },
                                        ),
                                    ),
                                    backtrack: BoundedBacktracker(
                                        None,
                                    ),
                                    onepass: OnePass(
                                        None,
                                    ),
                                    hybrid: Hybrid(
                                        Some(
                                            HybridEngine(
                                                Regex {
                                                    forward: DFA {
                                                        config: Config {
                                                            match_kind: Some(
                                                                All,
                                                            ),
                                                            pre: Some(
                                                                None,
                                                            ),
                                                            starts_for_each_pattern: Some(
                                                                true,
                                                            ),
                                                            byte_classes: Some(
                                                                true,
                                                            ),
                                                            unicode_word_boundary: Some(
                                                                true,
                                                            ),
                                                            quitset: None,
                                                            specialize_start_states: Some(
                                                                false,
                                                            ),
                                                            cache_capacity: Some(
                                                                10485760,
                                                            ),
                                                            skip_cache_capacity_check: Some(
                                                                false,
                                                            ),
                                                            minimum_cache_clear_count: Some(
                                                                Some(
                                                                    3,
                                                                ),
                                                            ),
                                                            minimum_bytes_per_state: Some(
                                                                Some(
                                                                    10,
                                                                ),
                                                            ),
                                                        },
                                                        nfa: thompson::NFA(
                                                        ^000000: FAIL

                                                        transition equivalence classes: ByteClasses(0 => [/x00-/xFF], 1 => [EOI])
                                                        )
                                                        ,
                                                        stride2: 1,
                                                        start_map: StartByteMap{/x00 => NonWordByte, /x01 => NonWordByte, /x02 => NonWordByte, /x03 => NonWordByte, /x04 => NonWordByte, /x05 => NonWordByte, /x06 => NonWordByte, /x07 => NonWordByte, /x08 => NonWordByte, /t => NonWordByte,
 => LineLF, /x0B => NonWordByte, /x0C => NonWordByte, /r => LineCR, /x0E => NonWordByte, /x0F => NonWordByte, /x10 => NonWordByte, /x11 => NonWordByte, /x12 => NonWordByte, /x13 => NonWordByte, /x14 => NonWordByte, /x15 => NonWordByte, /x16 => NonWordByte, /x17 => NonWordByte, /x18 => NonWordByte, /x19 => NonWordByte, /x1A => NonWordByte, /x1B => NonWordByte, /x1C => NonWordByte, /x1D => NonWordByte, /x1E => NonWordByte, /x1F => NonWordByte, ' ' => NonWordByte, ! => NonWordByte, /" => NonWordByte, # => NonWordByte, $ => NonWordByte, % => NonWordByte, & => NonWordByte, /' => NonWordByte, ( => NonWordByte, ) => NonWordByte, * => NonWordByte, + => NonWordByte, , => NonWordByte, - => NonWordByte, . => NonWordByte, / => NonWordByte, 0 => WordByte, 1 => WordByte, 2 => WordByte, 3 => WordByte, 4 => WordByte, 5 => WordByte, 6 => WordByte, 7 => WordByte, 8 => WordByte, 9 => WordByte, : => NonWordByte, ; => NonWordByte, < => NonWordByte, = => NonWordByte, > => NonWordByte, ? => NonWordByte, @ => NonWordByte, A => WordByte, B => WordByte, C => WordByte, D => WordByte, E => WordByte, F => WordByte, G => WordByte, H => WordByte, I => WordByte, J => WordByte, K => WordByte, L => WordByte, M => WordByte, N => WordByte, O => WordByte, P => WordByte, Q => WordByte, R => WordByte, S => WordByte, T => WordByte, U => WordByte, V => WordByte, W => WordByte, X => WordByte, Y => WordByte, Z => WordByte, [ => NonWordByte, // => NonWordByte, ] => NonWordByte, ^ => NonWordByte, _ => WordByte, ` => NonWordByte, a => WordByte, b => WordByte, c => WordByte, d => WordByte, e => WordByte, f => WordByte, g => WordByte, h => WordByte, i => WordByte, j => WordByte, k => WordByte, l => WordByte, m => WordByte, n => WordByte, o => WordByte, p => WordByte, q => WordByte, r => WordByte, s => WordByte, t => WordByte, u => WordByte, v => WordByte, w => WordByte, x => WordByte, y => WordByte, z => WordByte, { => NonWordByte, | => NonWordByte, } => NonWordByte, ~ => NonWordByte, /x7F => NonWordByte, /x80 => NonWordByte, /x81 => NonWordByte, /x82 => NonWordByte, /x83 => NonWordByte, /x84 => NonWordByte, /x85 => NonWordByte, /x86 => NonWordByte, /x87 => NonWordByte, /x88 => NonWordByte, /x89 => NonWordByte, /x8A => NonWordByte, /x8B => NonWordByte, /x8C => NonWordByte, /x8D => NonWordByte, /x8E => NonWordByte, /x8F => NonWordByte, /x90 => NonWordByte, /x91 => NonWordByte, /x92 => NonWordByte, /x93 => NonWordByte, /x94 => NonWordByte, /x95 => NonWordByte, /x96 => NonWordByte, /x97 => NonWordByte, /x98 => NonWordByte, /x99 => NonWordByte, /x9A => NonWordByte, /x9B => NonWordByte, /x9C => NonWordByte, /x9D => NonWordByte, /x9E => NonWordByte, /x9F => NonWordByte, /xA0 => NonWordByte, /xA1 => NonWordByte, /xA2 => NonWordByte, /xA3 => NonWordByte, /xA4 => NonWordByte, /xA5 => NonWordByte, /xA6 => NonWordByte, /xA7 => NonWordByte, /xA8 => NonWordByte, /xA9 => NonWordByte, /xAA => NonWordByte, /xAB => NonWordByte, /xAC => NonWordByte, /xAD => NonWordByte, /xAE => NonWordByte, /xAF => NonWordByte, /xB0 => NonWordByte, /xB1 => NonWordByte, /xB2 => NonWordByte, /xB3 => NonWordByte, /xB4 => NonWordByte, /xB5 => NonWordByte, /xB6 => NonWordByte, /xB7 => NonWordByte, /xB8 => NonWordByte, /xB9 => NonWordByte, /xBA => NonWordByte, /xBB => NonWordByte, /xBC => NonWordByte, /xBD => NonWordByte, /xBE => NonWordByte, /xBF => NonWordByte, /xC0 => NonWordByte, /xC1 => NonWordByte, /xC2 => NonWordByte, /xC3 => NonWordByte, /xC4 => NonWordByte, /xC5 => NonWordByte, /xC6 => NonWordByte, /xC7 => NonWordByte, /xC8 => NonWordByte, /xC9 => NonWordByte, /xCA => NonWordByte, /xCB => NonWordByte, /xCC => NonWordByte, /xCD => NonWordByte, /xCE => NonWordByte, /xCF => NonWordByte, /xD0 => NonWordByte, /xD1 => NonWordByte, /xD2 => NonWordByte, /xD3 => NonWordByte, /xD4 => NonWordByte, /xD5 => NonWordByte, /xD6 => NonWordByte, /xD7 => NonWordByte, /xD8 => NonWordByte, /xD9 => NonWordByte, /xDA => NonWordByte, /xDB => NonWordByte, /xDC => NonWordByte, /xDD => NonWordByte, /xDE => NonWordByte, /xDF => NonWordByte, /xE0 => NonWordByte, /xE1 => NonWordByte, /xE2 => NonWordByte, /xE3 => NonWordByte, /xE4 => NonWordByte, /xE5 => NonWordByte, /xE6 => NonWordByte, /xE7 => NonWordByte, /xE8 => NonWordByte, /xE9 => NonWordByte, /xEA => NonWordByte, /xEB => NonWordByte, /xEC => NonWordByte, /xED => NonWordByte, /xEE => NonWordByte, /xEF => NonWordByte, /xF0 => NonWordByte, /xF1 => NonWordByte, /xF2 => NonWordByte, /xF3 => NonWordByte, /xF4 => NonWordByte, /xF5 => NonWordByte, /xF6 => NonWordByte, /xF7 => NonWordByte, /xF8 => NonWordByte, /xF9 => NonWordByte, /xFA => NonWordByte, /xFB => NonWordByte, /xFC => NonWordByte, /xFD => NonWordByte, /xFE => NonWordByte, /xFF => NonWordByte},
                                                        classes: ByteClasses(0 => [/x00-/xFF], 1 => [EOI]),
                                                        quitset: ByteSet {
                                                            bits: {},
                                                        },
                                                        cache_capacity: 10485760,
                                                    },
                                                    reverse: DFA {
                                                        config: Config {
                                                            match_kind: Some(
                                                                All,
                                                            ),
                                                            pre: Some(
                                                                None,
                                                            ),
                                                            starts_for_each_pattern: Some(
                                                                true,
                                                            ),
                                                            byte_classes: Some(
                                                                true,
                                                            ),
                                                            unicode_word_boundary: Some(
                                                                true,
                                                            ),
                                                            quitset: None,
                                                            specialize_start_states: Some(
                                                                false,
                                                            ),
                                                            cache_capacity: Some(
                                                                10485760,
                                                            ),
                                                            skip_cache_capacity_check: Some(
                                                                false,
                                                            ),
                                                            minimum_cache_clear_count: Some(
                                                                Some(
                                                                    3,
                                                                ),
                                                            ),
                                                            minimum_bytes_per_state: Some(
                                                                Some(
                                                                    10,
                                                                ),
                                                            ),
                                                        },
                                                        nfa: thompson::NFA(
                                                        ^000000: FAIL

                                                        transition equivalence classes: ByteClasses(0 => [/x00-/xFF], 1 => [EOI])
                                                        )
                                                        ,
                                                        stride2: 1,
                                                        start_map: StartByteMap{/x00 => NonWordByte, /x01 => NonWordByte, /x02 => NonWordByte, /x03 => NonWordByte, /x04 => NonWordByte, /x05 => NonWordByte, /x06 => NonWordByte, /x07 => NonWordByte, /x08 => NonWordByte, /t => NonWordByte,
 => LineLF, /x0B => NonWordByte, /x0C => NonWordByte, /r => LineCR, /x0E => NonWordByte, /x0F => NonWordByte, /x10 => NonWordByte, /x11 => NonWordByte, /x12 => NonWordByte, /x13 => NonWordByte, /x14 => NonWordByte, /x15 => NonWordByte, /x16 => NonWordByte, /x17 => NonWordByte, /x18 => NonWordByte, /x19 => NonWordByte, /x1A => NonWordByte, /x1B => NonWordByte, /x1C => NonWordByte, /x1D => NonWordByte, /x1E => NonWordByte, /x1F => NonWordByte, ' ' => NonWordByte, ! => NonWordByte, /" => NonWordByte, # => NonWordByte, $ => NonWordByte, % => NonWordByte, & => NonWordByte, /' => NonWordByte, ( => NonWordByte, ) => NonWordByte, * => NonWordByte, + => NonWordByte, , => NonWordByte, - => NonWordByte, . => NonWordByte, / => NonWordByte, 0 => WordByte, 1 => WordByte, 2 => WordByte, 3 => WordByte, 4 => WordByte, 5 => WordByte, 6 => WordByte, 7 => WordByte, 8 => WordByte, 9 => WordByte, : => NonWordByte, ; => NonWordByte, < => NonWordByte, = => NonWordByte, > => NonWordByte, ? => NonWordByte, @ => NonWordByte, A => WordByte, B => WordByte, C => WordByte, D => WordByte, E => WordByte, F => WordByte, G => WordByte, H => WordByte, I => WordByte, J => WordByte, K => WordByte, L => WordByte, M => WordByte, N => WordByte, O => WordByte, P => WordByte, Q => WordByte, R => WordByte, S => WordByte, T => WordByte, U => WordByte, V => WordByte, W => WordByte, X => WordByte, Y => WordByte, Z => WordByte, [ => NonWordByte, // => NonWordByte, ] => NonWordByte, ^ => NonWordByte, _ => WordByte, ` => NonWordByte, a => WordByte, b => WordByte, c => WordByte, d => WordByte, e => WordByte, f => WordByte, g => WordByte, h => WordByte, i => WordByte, j => WordByte, k => WordByte, l => WordByte, m => WordByte, n => WordByte, o => WordByte, p => WordByte, q => WordByte, r => WordByte, s => WordByte, t => WordByte, u => WordByte, v => WordByte, w => WordByte, x => WordByte, y => WordByte, z => WordByte, { => NonWordByte, | => NonWordByte, } => NonWordByte, ~ => NonWordByte, /x7F => NonWordByte, /x80 => NonWordByte, /x81 => NonWordByte, /x82 => NonWordByte, /x83 => NonWordByte, /x84 => NonWordByte, /x85 => NonWordByte, /x86 => NonWordByte, /x87 => NonWordByte, /x88 => NonWordByte, /x89 => NonWordByte, /x8A => NonWordByte, /x8B => NonWordByte, /x8C => NonWordByte, /x8D => NonWordByte, /x8E => NonWordByte, /x8F => NonWordByte, /x90 => NonWordByte, /x91 => NonWordByte, /x92 => NonWordByte, /x93 => NonWordByte, /x94 => NonWordByte, /x95 => NonWordByte, /x96 => NonWordByte, /x97 => NonWordByte, /x98 => NonWordByte, /x99 => NonWordByte, /x9A => NonWordByte, /x9B => NonWordByte, /x9C => NonWordByte, /x9D => NonWordByte, /x9E => NonWordByte, /x9F => NonWordByte, /xA0 => NonWordByte, /xA1 => NonWordByte, /xA2 => NonWordByte, /xA3 => NonWordByte, /xA4 => NonWordByte, /xA5 => NonWordByte, /xA6 => NonWordByte, /xA7 => NonWordByte, /xA8 => NonWordByte, /xA9 => NonWordByte, /xAA => NonWordByte, /xAB => NonWordByte, /xAC => NonWordByte, /xAD => NonWordByte, /xAE => NonWordByte, /xAF => NonWordByte, /xB0 => NonWordByte, /xB1 => NonWordByte, /xB2 => NonWordByte, /xB3 => NonWordByte, /xB4 => NonWordByte, /xB5 => NonWordByte, /xB6 => NonWordByte, /xB7 => NonWordByte, /xB8 => NonWordByte, /xB9 => NonWordByte, /xBA => NonWordByte, /xBB => NonWordByte, /xBC => NonWordByte, /xBD => NonWordByte, /xBE => NonWordByte, /xBF => NonWordByte, /xC0 => NonWordByte, /xC1 => NonWordByte, /xC2 => NonWordByte, /xC3 => NonWordByte, /xC4 => NonWordByte, /xC5 => NonWordByte, /xC6 => NonWordByte, /xC7 => NonWordByte, /xC8 => NonWordByte, /xC9 => NonWordByte, /xCA => NonWordByte, /xCB => NonWordByte, /xCC => NonWordByte, /xCD => NonWordByte, /xCE => NonWordByte, /xCF => NonWordByte, /xD0 => NonWordByte, /xD1 => NonWordByte, /xD2 => NonWordByte, /xD3 => NonWordByte, /xD4 => NonWordByte, /xD5 => NonWordByte, /xD6 => NonWordByte, /xD7 => NonWordByte, /xD8 => NonWordByte, /xD9 => NonWordByte, /xDA => NonWordByte, /xDB => NonWordByte, /xDC => NonWordByte, /xDD => NonWordByte, /xDE => NonWordByte, /xDF => NonWordByte, /xE0 => NonWordByte, /xE1 => NonWordByte, /xE2 => NonWordByte, /xE3 => NonWordByte, /xE4 => NonWordByte, /xE5 => NonWordByte, /xE6 => NonWordByte, /xE7 => NonWordByte, /xE8 => NonWordByte, /xE9 => NonWordByte, /xEA => NonWordByte, /xEB => NonWordByte, /xEC => NonWordByte, /xED => NonWordByte, /xEE => NonWordByte, /xEF => NonWordByte, /xF0 => NonWordByte, /xF1 => NonWordByte, /xF2 => NonWordByte, /xF3 => NonWordByte, /xF4 => NonWordByte, /xF5 => NonWordByte, /xF6 => NonWordByte, /xF7 => NonWordByte, /xF8 => NonWordByte, /xF9 => NonWordByte, /xFA => NonWordByte, /xFB => NonWordByte, /xFC => NonWordByte, /xFD => NonWordByte, /xFE => NonWordByte, /xFF => NonWordByte},
                                                        classes: ByteClasses(0 => [/x00-/xFF], 1 => [EOI]),
                                                        quitset: ByteSet {
                                                            bits: {},
                                                        },
                                                        cache_capacity: 10485760,
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                    dfa: DFA(
                                        None,
                                    ),
                                },
                                info: RegexInfo(
                                    RegexInfoI {
                                        config: Config {
                                            match_kind: Some(
                                                All,
                                            ),
                                            utf8_empty: Some(
                                                false,
                                            ),
                                            autopre: None,
                                            pre: None,
                                            which_captures: None,
                                            nfa_size_limit: Some(
                                                Some(
                                                    10485760,
                                                ),
                                            ),
                                            onepass_size_limit: None,
                                            hybrid_cache_capacity: Some(
                                                10485760,
                                            ),
                                            hybrid: None,
                                            dfa: None,
                                            dfa_size_limit: None,
                                            dfa_state_limit: None,
                                            onepass: None,
                                            backtrack: None,
                                            byte_classes: None,
                                            line_terminator: None,
                                        },
                                        props: [],
                                        props_union: Properties(
                                            PropertiesI {
                                                minimum_len: None,
                                                maximum_len: None,
                                                look_set: ∅,
                                                look_set_prefix: ∅,
                                                look_set_suffix: ∅,
                                                look_set_prefix_any: ∅,
                                                look_set_suffix_any: ∅,
                                                utf8: true,
                                                explicit_captures_len: 0,
                                                static_explicit_captures_len: None,
                                                literal: false,
                                                alternation_literal: true,
                                            },
                                        ),
                                    },
                                ),
                            },
                            pool: Pool(
                                Pool {
                                    stacks: [
                                        CacheLine(
                                            Mutex {
                                                data: [],
                                                poisoned: false,
                                                ..
                                            },
                                        ),
                                        CacheLine(
                                            Mutex {
                                                data: [],
                                                poisoned: false,
                                                ..
                                            },
                                        ),
                                        CacheLine(
                                            Mutex {
                                                data: [],
                                                poisoned: false,
                                                ..
                                            },
                                        ),
                                        CacheLine(
                                            Mutex {
                                                data: [],
                                                poisoned: false,
                                                ..
                                            },
                                        ),
                                        CacheLine(
                                            Mutex {
                                                data: [],
                                                poisoned: false,
                                                ..
                                            },
                                        ),
                                        CacheLine(
                                            Mutex {
                                                data: [],
                                                poisoned: false,
                                                ..
                                            },
                                        ),
                                        CacheLine(
                                            Mutex {
                                                data: [],
                                                poisoned: false,
                                                ..
                                            },
                                        ),
                                        CacheLine(
                                            Mutex {
                                                data: [],
                                                poisoned: false,
                                                ..
                                            },
                                        ),
                                    ],
                                    owner: 0,
                                    owner_val: UnsafeCell { .. },
                                },
                            ),
                        },
                        map: [],
                        patset: Pool(
                            Pool {
                                stacks: [
                                    CacheLine(
                                        Mutex {
                                            data: [],
                                            poisoned: false,
                                            ..
                                        },
                                    ),
                                    CacheLine(
                                        Mutex {
                                            data: [],
                                            poisoned: false,
                                            ..
                                        },
                                    ),
                                    CacheLine(
                                        Mutex {
                                            data: [],
                                            poisoned: false,
                                            ..
                                        },
                                    ),
                                    CacheLine(
                                        Mutex {
                                            data: [],
                                            poisoned: false,
                                            ..
                                        },
                                    ),
                                    CacheLine(
                                        Mutex {
                                            data: [],
                                            poisoned: false,
                                            ..
                                        },
                                    ),
                                    CacheLine(
                                        Mutex {
                                            data: [],
                                            poisoned: false,
                                            ..
                                        },
                                    ),
                                    CacheLine(
                                        Mutex {
                                            data: [],
                                            poisoned: false,
                                            ..
                                        },
                                    ),
                                    CacheLine(
                                        Mutex {
                                            data: [],
                                            poisoned: false,
                                            ..
                                        },
                                    ),
                                ],
                                owner: 0,
                                owner_val: UnsafeCell { .. },
                            },
                        ),
                    },
                ),
            ],
        },
        poisoned: false,
        ..
    },
}
[TRACE][walker/src/notify.rs::170] watcher: FsEventWatcher {
    paths: 0x0000600001ae8000,
    since_when: 18446744073709551615,
    latency: 0.0,
    flags: 18,
    event_handler: 0x0000600003ff0010,
    runloop: Some(
        (
            0x00006000021fc0c0,
            JoinHandle { .. },
        ),
    ),
    recursive_info: {
        "[CWD]": true,
    },
}
[TRACE][core/src/util/pmp.rs::131] background_thread: Mutex {
    data: JoinHandle { .. },
    poisoned: false,
    ..
}
[TRACE][core/src/util/pmp.rs::59] watcher: FsEventWatcher {
    paths: 0x0000600001ae8000,
    since_when: 18446744073709551615,
    latency: 0.0,
    flags: 18,
    event_handler: 0x0000600003ff0010,
    runloop: Some(
        (
            0x00006000021fc0c0,
            JoinHandle { .. },
        ),
    ),
    recursive_info: {
        "[CWD]": true,
    },
}
[TRACE][core/src/util/pmp.rs::91] fs_event_index: 0
[TRACE][core/src/util/pmp.rs::93] kill_signal_index: 1
[TRACE][core/src/util/pmp.rs::96] "pmp background updater ticks": "pmp background updater ticks"
[TRACE][pipeline/src/pipeline/mod.rs::302] &pmp: XvcPathMetadataProvider {
    xvc_root: XvcRootInner {
        absolute_path: AbsolutePath(
            "[CWD]",
        ),
        xvc_dir: AbsolutePath(
            "[CWD]/.xvc",
        ),
        store_dir: AbsolutePath(
            "[CWD]/.xvc/store",
        ),
        config: XvcConfig {
            current_dir: XvcConfigOption {
                source: Runtime,
                option: AbsolutePath(
                    "[CWD]",
                ),
            },
            config_maps: [
                XvcConfigMap {
                    source: Default,
                    map: {
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "core.verbosity": String(
                            "error",
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "file.list.sort": String(
                            "name-desc",
                        ),
                        "git.use_git": Boolean(
                            true,
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "core.guid": String(
                            "919ce8649f3e32f7",
                        ),
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "pipeline.process_pool_size": Integer(
                            4,
                        ),
                        "git.command": String(
                            "git",
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "file.list.show_dot_files": Boolean(
                            false,
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                        "file.list.no_summary": Boolean(
                            false,
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                    },
                },
                XvcConfigMap {
                    source: Project,
                    map: {
                        "core.guid": String(
                            "59a2d95849757118",
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "git.command": String(
                            "git",
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "git.use_git": Boolean(
                            true,
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "file.list.no_summary": Boolean(
                            false,
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "file.list.show_dot_files": Boolean(
                            false,
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "file.list.sort": String(
                            "name-desc",
                        ),
                        "pipeline.process_pool_size": Integer(
                            4,
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "core.verbosity": String(
                            "error",
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                    },
                },
                XvcConfigMap {
                    source: Local,
                    map: {},
                },
                XvcConfigMap {
                    source: Environment,
                    map: {
                        "TRYCMD_TESTS": String(
                            "storage,file,pipeline,core",
                        ),
                    },
                },
                XvcConfigMap {
                    source: CommandLine,
                    map: {
                        "core.quiet": Boolean(
                            false,
                        ),
                        "core.verbosity": String(
                            "debug",
                        ),
                    },
                },
            ],
            the_config: {
                "pipeline.default_params_file": XvcConfigValue {
                    source: Project,
                    value: String(
                        "params.yaml",
                    ),
                },
                "file.list.sort": XvcConfigValue {
                    source: Project,
                    value: String(
                        "name-desc",
                    ),
                },
                "git.use_git": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        true,
                    ),
                },
                "git.auto_commit": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        true,
                    ),
                },
                "core.verbosity": XvcConfigValue {
                    source: CommandLine,
                    value: String(
                        "debug",
                    ),
                },
                "git.auto_stage": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "file.carry-in.force": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "pipeline.current_pipeline": XvcConfigValue {
                    source: Project,
                    value: String(
                        "default",
                    ),
                },
                "cache.algorithm": XvcConfigValue {
                    source: Project,
                    value: String(
                        "blake3",
                    ),
                },
                "pipeline.process_pool_size": XvcConfigValue {
                    source: Project,
                    value: Integer(
                        4,
                    ),
                },
                "file.list.no_summary": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "file.track.no_parallel": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "file.recheck.method": XvcConfigValue {
                    source: Project,
                    value: String(
                        "copy",
                    ),
                },
                "core.quiet": XvcConfigValue {
                    source: CommandLine,
                    value: Boolean(
                        false,
                    ),
                },
                "file.carry-in.no_parallel": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "core.guid": XvcConfigValue {
                    source: Project,
                    value: String(
                        "59a2d95849757118",
                    ),
                },
                "pipeline.default": XvcConfigValue {
                    source: Project,
                    value: String(
                        "default",
                    ),
                },
                "git.command": XvcConfigValue {
                    source: Project,
                    value: String(
                        "git",
                    ),
                },
                "file.track.force": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "file.track.no_commit": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "file.list.recursive": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "file.list.format": XvcConfigValue {
                    source: Project,
                    value: String(
                        "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                    ),
                },
                "file.track.text_or_binary": XvcConfigValue {
                    source: Project,
                    value: String(
                        "auto",
                    ),
                },
                "TRYCMD_TESTS": XvcConfigValue {
                    source: Environment,
                    value: String(
                        "storage,file,pipeline,core",
                    ),
                },
                "file.list.show_dot_files": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
            },
            init_params: XvcConfigParams {
                default_configuration: "
[core]
# The repository id. Please do not delete or change it.
# This is used to identify the repository and generate paths in storages.
# In the future it may be used to in other ways.
guid = /"919ce8649f3e32f7/"
# Default verbosity level.
# One of /"error/", /"warn/", /"info/"
verbosity = /"error/"

[git]
# Automate git operations.
# Turning this off leads Xvc to behave as if it's not in a Git repository.
# Not recommended unless you're really not using Git
use_git = true
# Command to run Git process.
# You can set this to an absolute path to specify an executable
# If set to a non-absolute path, the executable will be searched in $PATH.
command = /"git/"

# Commit changes in .xvc/ directory after commands.
# You can set this to false if you want to commit manually.
auto_commit = true

# Stage changes in .xvc/ directory without committing.
# auto_commit implies auto_stage.
# If you want to commit manually but don't want to stage after individual Xvc commands, you can set this to true.
auto_stage = false

[cache]
# The hash algorithm used for the cache.
# It may take blake3, blake2, sha2 or sha3 as values.
# All algorithms are selected to produce 256-bit hashes, so sha2 means SHA2-256, blake2 means BLAKE2s, etc.
# The cache path is produced by prepending algorithm name to the cache.
# Blake3 files are in .xvc/b3/, while sha2 files are in .xvc/s2/ etc.
algorithm = /"blake3/"

[file]

[file.track]

# Don't move file content to cache after xvc file track
no_commit = false
# Force to track files even if they are already tracked.
force = false

# Xvc calculates file content digest differently for text and binary files.
# This option controls whether to treat files as text or binary.
# It may take auto, text or binary as values.
# Auto check each file individually and treat it as text if it's text.
text_or_binary = /"auto/"

# Don't use parallelism in track operations.
# Note that some of the operations are implemented in parallel by default, and this option affects some heavier operations.
no_parallel = false

[file.list]

# Format for `xvc file list` rows. You can reorder or remove columns.
# The following are the keys for each row:
# - {acd64}:  actual content digest. All 64 digits from the workspace file's content.
# - {acd8}:  actual content digest. First 8 digits the file content digest.
# - {aft}:  actual file type. Whether the entry is a file (F), directory (D),
#   symlink (S), hardlink (H) or reflink (R).
# - {asz}:  actual size. The size of the workspace file in bytes. It uses MB,
#   GB and TB to represent sizes larger than 1MB.
# - {ats}:  actual timestamp. The timestamp of the workspace file.
# - {cst}:  cache status. One of /"=/", /">/", /"</", /"X/", or /"?/" to show
#   whether the file timestamp is the same as the cached timestamp, newer,
#   older, not cached or not tracked.
# - {name}: The name of the file or directory.
# - {rcd64}:  recorded content digest. All 64 digits.
# - {rcd8}:  recorded content digest. First 8 digits.
# - {rrm}:  recorded recheck method. Whether the entry is linked to the workspace
#   as a copy (C), symlink (S), hardlink (H) or reflink (R).
# - {rsz}:  recorded size. The size of the cached content in bytes. It uses
#   MB, GB and TB to represent sizes larged than 1MB.
# - {rts}:  recorded timestamp. The timestamp of the cached content.
#
# There are no escape sequences in the format string.
# If you want to add a tab, type it to the string.
# If you want to add a literal double curly brace, open an issue.
format = /"{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}/"

# Default sort order for `xvc file list`.
# Valid values are
# none, name-asc, name-desc, size-asc, size-desc, ts-asc, ts-desc.
sort = /"name-desc/"

# Show dot files like .gitignore
show_dot_files = false

# Do not show a summary for as the final row for `xvc file list`.
no_summary = false

# List files recursively always.
recursive = false

[file.carry-in]
# Carry-in the files to cache always, even if they are already present.
force = false

# Don't use parallel move/copy in carry-in
no_parallel = false

[file.recheck]
# The recheck method for Xvc. It may take copy, hardlink, symlink, reflink as values.
# The default is copy to make sure the options is portable.
# Copy duplicates the file content, while hardlink, symlink and reflink only create a new path to the file.
# Note that hardlink and symlink are read-only as they link the files in cache.
method = /"copy/"

[pipeline]
# Name of the current pipeline to run
current_pipeline = /"default/"
# Name of the default pipeline
default = /"default/"
# Name of the default params file name
default_params_file = /"params.yaml/"
# Number of command processes to run concurrently
process_pool_size = 4
#

",
                current_dir: AbsolutePath(
                    "[CWD]",
                ),
                include_system_config: true,
                include_user_config: true,
                project_config_path: Some(
                    AbsolutePath(
                        "[CWD]/.xvc/config.toml",
                    ),
                ),
                local_config_path: Some(
                    AbsolutePath(
                        "[CWD]/.xvc/config.local.toml",
                    ),
                ),
                include_environment_config: true,
                command_line_config: Some(
                    [
                        "core.verbosity = debug",
                        "core.quiet = false",
                    ],
                ),
            },
        },
        local_config_path: AbsolutePath(
            "[CWD]/.xvc/config.local.toml",
        ),
        project_config_path: AbsolutePath(
            "[CWD]/.xvc/config.toml",
        ),
        entity_generator: XvcEntityGenerator {
            counter: 4,
            random: 13513277701724141613,
            dirty: false,
        },
    },
    path_map: RwLock {
        data: {},
        poisoned: false,
        ..
    },
    kill_signal_sender: Sender { .. },
    background_thread: Mutex {
        data: JoinHandle { .. },
        poisoned: false,
        ..
    },
    output_sender: Sender { .. },
    ignore_rules: IgnoreRules {
        root: "[CWD]",
        ignore_patterns: RwLock {
            data: [
                Pattern {
                    pattern: Glob {
                        glob: "**/.xvc",
                        re: "(?-u)^(?:/?|.*/)//.xvc$",
                        opts: GlobOptions {
                            case_insensitive: false,
                            literal_separator: false,
                            backslash_escape: true,
                            empty_alternates: false,
                        },
                        tokens: Tokens(
                            [
                                RecursivePrefix,
                                Literal(
                                    '.',
                                ),
                                Literal(
                                    'x',
                                ),
                                Literal(
                                    'v',
                                ),
                                Literal(
                                    'c',
                                ),
                            ],
                        ),
                    },
                    original: ".xvc",
                    source: Global,
                    effect: Ignore,
                    relativity: Anywhere,
                    path_kind: Any,
                },
                Pattern {
                    pattern: Glob {
                        glob: "**/.git",
                        re: "(?-u)^(?:/?|.*/)//.git$",
                        opts: GlobOptions {
                            case_insensitive: false,
                            literal_separator: false,
                            backslash_escape: true,
                            empty_alternates: false,
                        },
                        tokens: Tokens(
                            [
                                RecursivePrefix,
                                Literal(
                                    '.',
                                ),
                                Literal(
                                    'g',
                                ),
                                Literal(
                                    'i',
                                ),
                                Literal(
                                    't',
                                ),
                            ],
                        ),
                    },
                    original: ".git",
                    source: Global,
                    effect: Ignore,
                    relativity: Anywhere,
                    path_kind: Any,
                },
                Pattern {
                    pattern: Glob {
                        glob: "**/.DS_Store",
                        re: "(?-u)^(?:/?|.*/)//.DS_Store$",
                        opts: GlobOptions {
                            case_insensitive: false,
                            literal_separator: false,
                            backslash_escape: true,
                            empty_alternates: false,
                        },
                        tokens: Tokens(
                            [
                                RecursivePrefix,
                                Literal(
                                    '.',
                                ),
                                Literal(
                                    'D',
                                ),
                                Literal(
                                    'S',
                                ),
                                Literal(
                                    '_',
                                ),
                                Literal(
                                    'S',
                                ),
                                Literal(
                                    't',
                                ),
                                Literal(
                                    'o',
                                ),
                                Literal(
                                    'r',
                                ),
                                Literal(
                                    'e',
                                ),
                            ],
                        ),
                    },
                    original: ".DS_Store",
                    source: File {
                        path: ".xvcignore",
                        line: 6,
                    },
                    effect: Ignore,
                    relativity: Anywhere,
                    path_kind: Any,
                },
            ],
            poisoned: false,
            ..
        },
        whitelist_patterns: RwLock {
            data: [],
            poisoned: false,
            ..
        },
        whitelist_set: RwLock {
            data: GlobSet {
                len: 0,
                strats: [],
            },
            poisoned: false,
            ..
        },
        ignore_set: RwLock {
            data: GlobSet {
                len: 3,
                strats: [
                    Extension(
                        ExtensionStrategy(
                            {},
                        ),
                    ),
                    BasenameLiteral(
                        BasenameLiteralStrategy(
                            {
                                [
                                    46,
                                    120,
                                    118,
                                    99,
                                ]: [
                                    0,
                                ],
                                [
                                    46,
                                    68,
                                    83,
                                    95,
                                    83,
                                    116,
                                    111,
                                    114,
                                    101,
                                ]: [
                                    2,
                                ],
                                [
                                    46,
                                    103,
                                    105,
                                    116,
                                ]: [
                                    1,
                                ],
                            },
                        ),
                    ),
                    Literal(
                        LiteralStrategy(
                            {},
                        ),
                    ),
                    Suffix(
                        SuffixStrategy {
                            matcher: AhoCorasick(
                                dfa::DFA(
                                D 000000: /x00 => 0
                                F 000001:
                                 >000002: /x00 => 2
                                  000003: /x00 => 0
                                match kind: Standard
                                prefilter: false
                                state length: 4
                                pattern length: 0
                                shortest pattern length: 18446744073709551615
                                longest pattern length: 0
                                alphabet length: 1
                                stride: 1
                                byte classes: ByteClasses(0 => [0-255])
                                memory usage: 16
                                )
                                ,
                            ),
                            map: [],
                            longest: 0,
                        },
                    ),
                    Prefix(
                        PrefixStrategy {
                            matcher: AhoCorasick(
                                dfa::DFA(
                                D 000000: /x00 => 0
                                F 000001:
                                 >000002: /x00 => 2
                                  000003: /x00 => 0
                                match kind: Standard
                                prefilter: false
                                state length: 4
                                pattern length: 0
                                shortest pattern length: 18446744073709551615
                                longest pattern length: 0
                                alphabet length: 1
                                stride: 1
                                byte classes: ByteClasses(0 => [0-255])
                                memory usage: 16
                                )
                                ,
                            ),
                            map: [],
                            longest: 0,
                        },
                    ),
                    RequiredExtension(
                        RequiredExtensionStrategy(
                            {},
                        ),
                    ),
                    Regex(
                        RegexSetStrategy {
                            matcher: Regex {
                                imp: RegexI {
                                    strat: Core {
                                        info: RegexInfo(
                                            RegexInfoI {
                                                config: Config {
                                                    match_kind: Some(
                                                        All,
                                                    ),
                                                    utf8_empty: Some(
                                                        false,
                                                    ),
                                                    autopre: None,
                                                    pre: None,
                                                    which_captures: None,
                                                    nfa_size_limit: Some(
                                                        Some(
                                                            10485760,
                                                        ),
                                                    ),
                                                    onepass_size_limit: None,
                                                    hybrid_cache_capacity: Some(
                                                        10485760,
                                                    ),
                                                    hybrid: None,
                                                    dfa: None,
                                                    dfa_size_limit: None,
                                                    dfa_state_limit: None,
                                                    onepass: None,
                                                    backtrack: None,
                                                    byte_classes: None,
                                                    line_terminator: None,
                                                },
                                                props: [],
                                                props_union: Properties(
                                                    PropertiesI {
                                                        minimum_len: None,
                                                        maximum_len: None,
                                                        look_set: ∅,
                                                        look_set_prefix: ∅,
                                                        look_set_suffix: ∅,
                                                        look_set_prefix_any: ∅,
                                                        look_set_suffix_any: ∅,
                                                        utf8: true,
                                                        explicit_captures_len: 0,
                                                        static_explicit_captures_len: None,
                                                        literal: false,
                                                        alternation_literal: true,
                                                    },
                                                ),
                                            },
                                        ),
                                        pre: None,
                                        nfa: thompson::NFA(
                                        ^000000: FAIL

                                        transition equivalence classes: ByteClasses(0 => [/x00-/xFF], 1 => [EOI])
                                        )
                                        ,
                                        nfarev: Some(
                                            thompson::NFA(
                                            ^000000: FAIL

                                            transition equivalence classes: ByteClasses(0 => [/x00-/xFF], 1 => [EOI])
                                            )
                                            ,
                                        ),
                                        pikevm: PikeVM(
                                            PikeVMEngine(
                                                PikeVM {
                                                    config: Config {
                                                        match_kind: Some(
                                                            All,
                                                        ),
                                                        pre: Some(
                                                            None,
                                                        ),
                                                    },
                                                    nfa: thompson::NFA(
                                                    ^000000: FAIL

                                                    transition equivalence classes: ByteClasses(0 => [/x00-/xFF], 1 => [EOI])
                                                    )
                                                    ,
                                                },
                                            ),
                                        ),
                                        backtrack: BoundedBacktracker(
                                            None,
                                        ),
                                        onepass: OnePass(
                                            None,
                                        ),
                                        hybrid: Hybrid(
                                            Some(
                                                HybridEngine(
                                                    Regex {
                                                        forward: DFA {
                                                            config: Config {
                                                                match_kind: Some(
                                                                    All,
                                                                ),
                                                                pre: Some(
                                                                    None,
                                                                ),
                                                                starts_for_each_pattern: Some(
                                                                    true,
                                                                ),
                                                                byte_classes: Some(
                                                                    true,
                                                                ),
                                                                unicode_word_boundary: Some(
                                                                    true,
                                                                ),
                                                                quitset: None,
                                                                specialize_start_states: Some(
                                                                    false,
                                                                ),
                                                                cache_capacity: Some(
                                                                    10485760,
                                                                ),
                                                                skip_cache_capacity_check: Some(
                                                                    false,
                                                                ),
                                                                minimum_cache_clear_count: Some(
                                                                    Some(
                                                                        3,
                                                                    ),
                                                                ),
                                                                minimum_bytes_per_state: Some(
                                                                    Some(
                                                                        10,
                                                                    ),
                                                                ),
                                                            },
                                                            nfa: thompson::NFA(
                                                            ^000000: FAIL

                                                            transition equivalence classes: ByteClasses(0 => [/x00-/xFF], 1 => [EOI])
                                                            )
                                                            ,
                                                            stride2: 1,
                                                            start_map: StartByteMap{/x00 => NonWordByte, /x01 => NonWordByte, /x02 => NonWordByte, /x03 => NonWordByte, /x04 => NonWordByte, /x05 => NonWordByte, /x06 => NonWordByte, /x07 => NonWordByte, /x08 => NonWordByte, /t => NonWordByte,
 => LineLF, /x0B => NonWordByte, /x0C => NonWordByte, /r => LineCR, /x0E => NonWordByte, /x0F => NonWordByte, /x10 => NonWordByte, /x11 => NonWordByte, /x12 => NonWordByte, /x13 => NonWordByte, /x14 => NonWordByte, /x15 => NonWordByte, /x16 => NonWordByte, /x17 => NonWordByte, /x18 => NonWordByte, /x19 => NonWordByte, /x1A => NonWordByte, /x1B => NonWordByte, /x1C => NonWordByte, /x1D => NonWordByte, /x1E => NonWordByte, /x1F => NonWordByte, ' ' => NonWordByte, ! => NonWordByte, /" => NonWordByte, # => NonWordByte, $ => NonWordByte, % => NonWordByte, & => NonWordByte, /' => NonWordByte, ( => NonWordByte, ) => NonWordByte, * => NonWordByte, + => NonWordByte, , => NonWordByte, - => NonWordByte, . => NonWordByte, / => NonWordByte, 0 => WordByte, 1 => WordByte, 2 => WordByte, 3 => WordByte, 4 => WordByte, 5 => WordByte, 6 => WordByte, 7 => WordByte, 8 => WordByte, 9 => WordByte, : => NonWordByte, ; => NonWordByte, < => NonWordByte, = => NonWordByte, > => NonWordByte, ? => NonWordByte, @ => NonWordByte, A => WordByte, B => WordByte, C => WordByte, D => WordByte, E => WordByte, F => WordByte, G => WordByte, H => WordByte, I => WordByte, J => WordByte, K => WordByte, L => WordByte, M => WordByte, N => WordByte, O => WordByte, P => WordByte, Q => WordByte, R => WordByte, S => WordByte, T => WordByte, U => WordByte, V => WordByte, W => WordByte, X => WordByte, Y => WordByte, Z => WordByte, [ => NonWordByte, // => NonWordByte, ] => NonWordByte, ^ => NonWordByte, _ => WordByte, ` => NonWordByte, a => WordByte, b => WordByte, c => WordByte, d => WordByte, e => WordByte, f => WordByte, g => WordByte, h => WordByte, i => WordByte, j => WordByte, k => WordByte, l => WordByte, m => WordByte, n => WordByte, o => WordByte, p => WordByte, q => WordByte, r => WordByte, s => WordByte, t => WordByte, u => WordByte, v => WordByte, w => WordByte, x => WordByte, y => WordByte, z => WordByte, { => NonWordByte, | => NonWordByte, } => NonWordByte, ~ => NonWordByte, /x7F => NonWordByte, /x80 => NonWordByte, /x81 => NonWordByte, /x82 => NonWordByte, /x83 => NonWordByte, /x84 => NonWordByte, /x85 => NonWordByte, /x86 => NonWordByte, /x87 => NonWordByte, /x88 => NonWordByte, /x89 => NonWordByte, /x8A => NonWordByte, /x8B => NonWordByte, /x8C => NonWordByte, /x8D => NonWordByte, /x8E => NonWordByte, /x8F => NonWordByte, /x90 => NonWordByte, /x91 => NonWordByte, /x92 => NonWordByte, /x93 => NonWordByte, /x94 => NonWordByte, /x95 => NonWordByte, /x96 => NonWordByte, /x97 => NonWordByte, /x98 => NonWordByte, /x99 => NonWordByte, /x9A => NonWordByte, /x9B => NonWordByte, /x9C => NonWordByte, /x9D => NonWordByte, /x9E => NonWordByte, /x9F => NonWordByte, /xA0 => NonWordByte, /xA1 => NonWordByte, /xA2 => NonWordByte, /xA3 => NonWordByte, /xA4 => NonWordByte, /xA5 => NonWordByte, /xA6 => NonWordByte, /xA7 => NonWordByte, /xA8 => NonWordByte, /xA9 => NonWordByte, /xAA => NonWordByte, /xAB => NonWordByte, /xAC => NonWordByte, /xAD => NonWordByte, /xAE => NonWordByte, /xAF => NonWordByte, /xB0 => NonWordByte, /xB1 => NonWordByte, /xB2 => NonWordByte, /xB3 => NonWordByte, /xB4 => NonWordByte, /xB5 => NonWordByte, /xB6 => NonWordByte, /xB7 => NonWordByte, /xB8 => NonWordByte, /xB9 => NonWordByte, /xBA => NonWordByte, /xBB => NonWordByte, /xBC => NonWordByte, /xBD => NonWordByte, /xBE => NonWordByte, /xBF => NonWordByte, /xC0 => NonWordByte, /xC1 => NonWordByte, /xC2 => NonWordByte, /xC3 => NonWordByte, /xC4 => NonWordByte, /xC5 => NonWordByte, /xC6 => NonWordByte, /xC7 => NonWordByte, /xC8 => NonWordByte, /xC9 => NonWordByte, /xCA => NonWordByte, /xCB => NonWordByte, /xCC => NonWordByte, /xCD => NonWordByte, /xCE => NonWordByte, /xCF => NonWordByte, /xD0 => NonWordByte, /xD1 => NonWordByte, /xD2 => NonWordByte, /xD3 => NonWordByte, /xD4 => NonWordByte, /xD5 => NonWordByte, /xD6 => NonWordByte, /xD7 => NonWordByte, /xD8 => NonWordByte, /xD9 => NonWordByte, /xDA => NonWordByte, /xDB => NonWordByte, /xDC => NonWordByte, /xDD => NonWordByte, /xDE => NonWordByte, /xDF => NonWordByte, /xE0 => NonWordByte, /xE1 => NonWordByte, /xE2 => NonWordByte, /xE3 => NonWordByte, /xE4 => NonWordByte, /xE5 => NonWordByte, /xE6 => NonWordByte, /xE7 => NonWordByte, /xE8 => NonWordByte, /xE9 => NonWordByte, /xEA => NonWordByte, /xEB => NonWordByte, /xEC => NonWordByte, /xED => NonWordByte, /xEE => NonWordByte, /xEF => NonWordByte, /xF0 => NonWordByte, /xF1 => NonWordByte, /xF2 => NonWordByte, /xF3 => NonWordByte, /xF4 => NonWordByte, /xF5 => NonWordByte, /xF6 => NonWordByte, /xF7 => NonWordByte, /xF8 => NonWordByte, /xF9 => NonWordByte, /xFA => NonWordByte, /xFB => NonWordByte, /xFC => NonWordByte, /xFD => NonWordByte, /xFE => NonWordByte, /xFF => NonWordByte},
                                                            classes: ByteClasses(0 => [/x00-/xFF], 1 => [EOI]),
                                                            quitset: ByteSet {
                                                                bits: {},
                                                            },
                                                            cache_capacity: 10485760,
                                                        },
                                                        reverse: DFA {
                                                            config: Config {
                                                                match_kind: Some(
                                                                    All,
                                                                ),
                                                                pre: Some(
                                                                    None,
                                                                ),
                                                                starts_for_each_pattern: Some(
                                                                    true,
                                                                ),
                                                                byte_classes: Some(
                                                                    true,
                                                                ),
                                                                unicode_word_boundary: Some(
                                                                    true,
                                                                ),
                                                                quitset: None,
                                                                specialize_start_states: Some(
                                                                    false,
                                                                ),
                                                                cache_capacity: Some(
                                                                    10485760,
                                                                ),
                                                                skip_cache_capacity_check: Some(
                                                                    false,
                                                                ),
                                                                minimum_cache_clear_count: Some(
                                                                    Some(
                                                                        3,
                                                                    ),
                                                                ),
                                                                minimum_bytes_per_state: Some(
                                                                    Some(
                                                                        10,
                                                                    ),
                                                                ),
                                                            },
                                                            nfa: thompson::NFA(
                                                            ^000000: FAIL

                                                            transition equivalence classes: ByteClasses(0 => [/x00-/xFF], 1 => [EOI])
                                                            )
                                                            ,
                                                            stride2: 1,
                                                            start_map: StartByteMap{/x00 => NonWordByte, /x01 => NonWordByte, /x02 => NonWordByte, /x03 => NonWordByte, /x04 => NonWordByte, /x05 => NonWordByte, /x06 => NonWordByte, /x07 => NonWordByte, /x08 => NonWordByte, /t => NonWordByte,
 => LineLF, /x0B => NonWordByte, /x0C => NonWordByte, /r => LineCR, /x0E => NonWordByte, /x0F => NonWordByte, /x10 => NonWordByte, /x11 => NonWordByte, /x12 => NonWordByte, /x13 => NonWordByte, /x14 => NonWordByte, /x15 => NonWordByte, /x16 => NonWordByte, /x17 => NonWordByte, /x18 => NonWordByte, /x19 => NonWordByte, /x1A => NonWordByte, /x1B => NonWordByte, /x1C => NonWordByte, /x1D => NonWordByte, /x1E => NonWordByte, /x1F => NonWordByte, ' ' => NonWordByte, ! => NonWordByte, /" => NonWordByte, # => NonWordByte, $ => NonWordByte, % => NonWordByte, & => NonWordByte, /' => NonWordByte, ( => NonWordByte, ) => NonWordByte, * => NonWordByte, + => NonWordByte, , => NonWordByte, - => NonWordByte, . => NonWordByte, / => NonWordByte, 0 => WordByte, 1 => WordByte, 2 => WordByte, 3 => WordByte, 4 => WordByte, 5 => WordByte, 6 => WordByte, 7 => WordByte, 8 => WordByte, 9 => WordByte, : => NonWordByte, ; => NonWordByte, < => NonWordByte, = => NonWordByte, > => NonWordByte, ? => NonWordByte, @ => NonWordByte, A => WordByte, B => WordByte, C => WordByte, D => WordByte, E => WordByte, F => WordByte, G => WordByte, H => WordByte, I => WordByte, J => WordByte, K => WordByte, L => WordByte, M => WordByte, N => WordByte, O => WordByte, P => WordByte, Q => WordByte, R => WordByte, S => WordByte, T => WordByte, U => WordByte, V => WordByte, W => WordByte, X => WordByte, Y => WordByte, Z => WordByte, [ => NonWordByte, // => NonWordByte, ] => NonWordByte, ^ => NonWordByte, _ => WordByte, ` => NonWordByte, a => WordByte, b => WordByte, c => WordByte, d => WordByte, e => WordByte, f => WordByte, g => WordByte, h => WordByte, i => WordByte, j => WordByte, k => WordByte, l => WordByte, m => WordByte, n => WordByte, o => WordByte, p => WordByte, q => WordByte, r => WordByte, s => WordByte, t => WordByte, u => WordByte, v => WordByte, w => WordByte, x => WordByte, y => WordByte, z => WordByte, { => NonWordByte, | => NonWordByte, } => NonWordByte, ~ => NonWordByte, /x7F => NonWordByte, /x80 => NonWordByte, /x81 => NonWordByte, /x82 => NonWordByte, /x83 => NonWordByte, /x84 => NonWordByte, /x85 => NonWordByte, /x86 => NonWordByte, /x87 => NonWordByte, /x88 => NonWordByte, /x89 => NonWordByte, /x8A => NonWordByte, /x8B => NonWordByte, /x8C => NonWordByte, /x8D => NonWordByte, /x8E => NonWordByte, /x8F => NonWordByte, /x90 => NonWordByte, /x91 => NonWordByte, /x92 => NonWordByte, /x93 => NonWordByte, /x94 => NonWordByte, /x95 => NonWordByte, /x96 => NonWordByte, /x97 => NonWordByte, /x98 => NonWordByte, /x99 => NonWordByte, /x9A => NonWordByte, /x9B => NonWordByte, /x9C => NonWordByte, /x9D => NonWordByte, /x9E => NonWordByte, /x9F => NonWordByte, /xA0 => NonWordByte, /xA1 => NonWordByte, /xA2 => NonWordByte, /xA3 => NonWordByte, /xA4 => NonWordByte, /xA5 => NonWordByte, /xA6 => NonWordByte, /xA7 => NonWordByte, /xA8 => NonWordByte, /xA9 => NonWordByte, /xAA => NonWordByte, /xAB => NonWordByte, /xAC => NonWordByte, /xAD => NonWordByte, /xAE => NonWordByte, /xAF => NonWordByte, /xB0 => NonWordByte, /xB1 => NonWordByte, /xB2 => NonWordByte, /xB3 => NonWordByte, /xB4 => NonWordByte, /xB5 => NonWordByte, /xB6 => NonWordByte, /xB7 => NonWordByte, /xB8 => NonWordByte, /xB9 => NonWordByte, /xBA => NonWordByte, /xBB => NonWordByte, /xBC => NonWordByte, /xBD => NonWordByte, /xBE => NonWordByte, /xBF => NonWordByte, /xC0 => NonWordByte, /xC1 => NonWordByte, /xC2 => NonWordByte, /xC3 => NonWordByte, /xC4 => NonWordByte, /xC5 => NonWordByte, /xC6 => NonWordByte, /xC7 => NonWordByte, /xC8 => NonWordByte, /xC9 => NonWordByte, /xCA => NonWordByte, /xCB => NonWordByte, /xCC => NonWordByte, /xCD => NonWordByte, /xCE => NonWordByte, /xCF => NonWordByte, /xD0 => NonWordByte, /xD1 => NonWordByte, /xD2 => NonWordByte, /xD3 => NonWordByte, /xD4 => NonWordByte, /xD5 => NonWordByte, /xD6 => NonWordByte, /xD7 => NonWordByte, /xD8 => NonWordByte, /xD9 => NonWordByte, /xDA => NonWordByte, /xDB => NonWordByte, /xDC => NonWordByte, /xDD => NonWordByte, /xDE => NonWordByte, /xDF => NonWordByte, /xE0 => NonWordByte, /xE1 => NonWordByte, /xE2 => NonWordByte, /xE3 => NonWordByte, /xE4 => NonWordByte, /xE5 => NonWordByte, /xE6 => NonWordByte, /xE7 => NonWordByte, /xE8 => NonWordByte, /xE9 => NonWordByte, /xEA => NonWordByte, /xEB => NonWordByte, /xEC => NonWordByte, /xED => NonWordByte, /xEE => NonWordByte, /xEF => NonWordByte, /xF0 => NonWordByte, /xF1 => NonWordByte, /xF2 => NonWordByte, /xF3 => NonWordByte, /xF4 => NonWordByte, /xF5 => NonWordByte, /xF6 => NonWordByte, /xF7 => NonWordByte, /xF8 => NonWordByte, /xF9 => NonWordByte, /xFA => NonWordByte, /xFB => NonWordByte, /xFC => NonWordByte, /xFD => NonWordByte, /xFE => NonWordByte, /xFF => NonWordByte},
                                                            classes: ByteClasses(0 => [/x00-/xFF], 1 => [EOI]),
                                                            quitset: ByteSet {
                                                                bits: {},
                                                            },
                                                            cache_capacity: 10485760,
                                                        },
                                                    },
                                                ),
                                            ),
                                        ),
                                        dfa: DFA(
                                            None,
                                        ),
                                    },
                                    info: RegexInfo(
                                        RegexInfoI {
                                            config: Config {
                                                match_kind: Some(
                                                    All,
                                                ),
                                                utf8_empty: Some(
                                                    false,
                                                ),
                                                autopre: None,
                                                pre: None,
                                                which_captures: None,
                                                nfa_size_limit: Some(
                                                    Some(
                                                        10485760,
                                                    ),
                                                ),
                                                onepass_size_limit: None,
                                                hybrid_cache_capacity: Some(
                                                    10485760,
                                                ),
                                                hybrid: None,
                                                dfa: None,
                                                dfa_size_limit: None,
                                                dfa_state_limit: None,
                                                onepass: None,
                                                backtrack: None,
                                                byte_classes: None,
                                                line_terminator: None,
                                            },
                                            props: [],
                                            props_union: Properties(
                                                PropertiesI {
                                                    minimum_len: None,
                                                    maximum_len: None,
                                                    look_set: ∅,
                                                    look_set_prefix: ∅,
                                                    look_set_suffix: ∅,
                                                    look_set_prefix_any: ∅,
                                                    look_set_suffix_any: ∅,
                                                    utf8: true,
                                                    explicit_captures_len: 0,
                                                    static_explicit_captures_len: None,
                                                    literal: false,
                                                    alternation_literal: true,
                                                },
                                            ),
                                        },
                                    ),
                                },
                                pool: Pool(
                                    Pool {
                                        stacks: [
                                            CacheLine(
                                                Mutex {
                                                    data: [],
                                                    poisoned: false,
                                                    ..
                                                },
                                            ),
                                            CacheLine(
                                                Mutex {
                                                    data: [],
                                                    poisoned: false,
                                                    ..
                                                },
                                            ),
                                            CacheLine(
                                                Mutex {
                                                    data: [],
                                                    poisoned: false,
                                                    ..
                                                },
                                            ),
                                            CacheLine(
                                                Mutex {
                                                    data: [],
                                                    poisoned: false,
                                                    ..
                                                },
                                            ),
                                            CacheLine(
                                                Mutex {
                                                    data: [],
                                                    poisoned: false,
                                                    ..
                                                },
                                            ),
                                            CacheLine(
                                                Mutex {
                                                    data: [],
                                                    poisoned: false,
                                                    ..
                                                },
                                            ),
                                            CacheLine(
                                                Mutex {
                                                    data: [],
                                                    poisoned: false,
                                                    ..
                                                },
                                            ),
                                            CacheLine(
                                                Mutex {
                                                    data: [],
                                                    poisoned: false,
                                                    ..
                                                },
                                            ),
                                        ],
                                        owner: 0,
                                        owner_val: UnsafeCell { .. },
                                    },
                                ),
                            },
                            map: [],
                            patset: Pool(
                                Pool {
                                    stacks: [
                                        CacheLine(
                                            Mutex {
                                                data: [],
                                                poisoned: false,
                                                ..
                                            },
                                        ),
                                        CacheLine(
                                            Mutex {
                                                data: [],
                                                poisoned: false,
                                                ..
                                            },
                                        ),
                                        CacheLine(
                                            Mutex {
                                                data: [],
                                                poisoned: false,
                                                ..
                                            },
                                        ),
                                        CacheLine(
                                            Mutex {
                                                data: [],
                                                poisoned: false,
                                                ..
                                            },
                                        ),
                                        CacheLine(
                                            Mutex {
                                                data: [],
                                                poisoned: false,
                                                ..
                                            },
                                        ),
                                        CacheLine(
                                            Mutex {
                                                data: [],
                                                poisoned: false,
                                                ..
                                            },
                                        ),
                                        CacheLine(
                                            Mutex {
                                                data: [],
                                                poisoned: false,
                                                ..
                                            },
                                        ),
                                        CacheLine(
                                            Mutex {
                                                data: [],
                                                poisoned: false,
                                                ..
                                            },
                                        ),
                                    ],
                                    owner: 0,
                                    owner_val: UnsafeCell { .. },
                                },
                            ),
                        },
                    ),
                ],
            },
            poisoned: false,
            ..
        },
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::305] pipeline_len: 1
[TRACE][pipeline/src/pipeline/mod.rs::331] &dependency_graph: {
    XvcEntity(
        2,
        9270302598983016314,
    ): [],
}
[TRACE][pipeline/src/pipeline/mod.rs::343] &dependency_graph: {
    XvcEntity(
        2,
        9270302598983016314,
    ): [],
}
[INFO][pipeline/src/pipeline/mod.rs::347] Pipeline Graph:
digraph {
    0 [ label = "(2, 9270302598983016314)" ]
}


[TRACE][pipeline/src/pipeline/mod.rs::412] step_states: RwLock {
    data: HStore {
        map: {
            XvcEntity(
                2,
                9270302598983016314,
            ): Begin(
                FromInit,
            ),
        },
    },
    poisoned: false,
    ..
}
[TRACE][pipeline/src/pipeline/mod.rs::508] &step_thread_store: HStore {
    map: {
        XvcEntity(
            2,
            9270302598983016314,
        ): ScopedJoinHandle { .. },
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::512] (step_e, &jh): (
    XvcEntity(
        2,
        9270302598983016314,
    ),
    ScopedJoinHandle { .. },
)
[TRACE][pipeline/src/pipeline/mod.rs::621] params.recorded_dependencies: R1NStore {
    parents: XvcStore {
        map: {
            XvcEntity(
                2,
                9270302598983016314,
            ): XvcStep {
                name: "average-age",
            },
        },
        entity_index: {
            XvcStep {
                name: "average-age",
            }: [
                XvcEntity(
                    2,
                    9270302598983016314,
                ),
            ],
        },
        previous: EventLog(
            [
                Add {
                    entity: XvcEntity(
                        2,
                        9270302598983016314,
                    ),
                    value: XvcStep {
                        name: "average-age",
                    },
                },
                Add {
                    entity: XvcEntity(
                        2,
                        9270302598983016314,
                    ),
                    value: XvcStep {
                        name: "average-age",
                    },
                },
            ],
        ),
        current: EventLog(
            [],
        ),
    },
    children: XvcStore {
        map: {
            XvcEntity(
                3,
                9022053985516504033,
            ): SqliteQueryDigest(
                SqliteQueryDep {
                    path: XvcPath(
                        "people.db",
                    ),
                    query: "SELECT count(*) FROM People;",
                    query_digest: None,
                    xvc_metadata: None,
                },
            ),
        },
        entity_index: {
            SqliteQueryDigest(
                SqliteQueryDep {
                    path: XvcPath(
                        "people.db",
                    ),
                    query: "SELECT count(*) FROM People;",
                    query_digest: None,
                    xvc_metadata: None,
                },
            ): [
                XvcEntity(
                    3,
                    9022053985516504033,
                ),
            ],
        },
        previous: EventLog(
            [
                Add {
                    entity: XvcEntity(
                        3,
                        9022053985516504033,
                    ),
                    value: SqliteQueryDigest(
                        SqliteQueryDep {
                            path: XvcPath(
                                "people.db",
                            ),
                            query: "SELECT count(*) FROM People;",
                            query_digest: None,
                            xvc_metadata: None,
                        },
                    ),
                },
            ],
        ),
        current: EventLog(
            [],
        ),
    },
    child_parents: XvcStore {
        map: {
            XvcEntity(
                3,
                9022053985516504033,
            ): ChildEntity(
                XvcEntity(
                    2,
                    9270302598983016314,
                ),
                PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
            ),
        },
        entity_index: {
            ChildEntity(
                XvcEntity(
                    2,
                    9270302598983016314,
                ),
                PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
            ): [
                XvcEntity(
                    3,
                    9022053985516504033,
                ),
            ],
        },
        previous: EventLog(
            [
                Add {
                    entity: XvcEntity(
                        3,
                        9022053985516504033,
                    ),
                    value: ChildEntity(
                        XvcEntity(
                            2,
                            9270302598983016314,
                        ),
                        PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                        PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                    ),
                },
            ],
        ),
        current: EventLog(
            [],
        ),
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::622] step_e: XvcEntity(
    2,
    9270302598983016314,
)
[TRACE][pipeline/src/pipeline/mod.rs::564] dep_neighbors: Neighbors {
    iter: Iter(
        [],
    ),
    ty: PhantomData<petgraph::Directed>,
}
[TRACE][pipeline/src/pipeline/mod.rs::623] dependency_steps(step_e, params.dependency_graph)?: {}
[TRACE][pipeline/src/pipeline/mod.rs::564] dep_neighbors: Neighbors {
    iter: Iter(
        [],
    ),
    ty: PhantomData<petgraph::Directed>,
}
[TRACE][pipeline/src/pipeline/mod.rs::660] &step_state: Begin(
    FromInit,
)
[TRACE][pipeline/src/pipeline/mod.rs::770] step.name: "average-age"
[TRACE][pipeline/src/pipeline/mod.rs::771] &r_next_state: WaitingDependencySteps(
    FromRunConditional,
)
[TRACE][pipeline/src/pipeline/mod.rs::773] &step_state: WaitingDependencySteps(
    FromRunConditional,
)
[TRACE][pipeline/src/pipeline/mod.rs::660] &step_state: WaitingDependencySteps(
    FromRunConditional,
)
[TRACE][pipeline/src/pipeline/mod.rs::770] step.name: "average-age"
[INFO] No dependency steps for step average-age
[TRACE][pipeline/src/pipeline/mod.rs::771] &r_next_state: CheckingOutputs(
    FromDependencyStepsFinishedSuccessfully,
)
[TRACE][pipeline/src/pipeline/mod.rs::773] &step_state: CheckingOutputs(
    FromDependencyStepsFinishedSuccessfully,
)
[TRACE][pipeline/src/pipeline/mod.rs::660] &step_state: CheckingOutputs(
    FromDependencyStepsFinishedSuccessfully,
)
[TRACE][pipeline/src/pipeline/mod.rs::770] step.name: "average-age"
[TRACE][pipeline/src/pipeline/mod.rs::771] &r_next_state: CheckingSuperficialDiffs(
    FromCheckedOutputs,
)
[TRACE][pipeline/src/pipeline/mod.rs::773] &step_state: CheckingSuperficialDiffs(
    FromCheckedOutputs,
)
[TRACE][pipeline/src/pipeline/mod.rs::660] &step_state: CheckingSuperficialDiffs(
    FromCheckedOutputs,
)
[TRACE][pipeline/src/pipeline/mod.rs::1008] parent_entity: XvcEntity(
    2,
    9270302598983016314,
)
[TRACE][pipeline/src/pipeline/mod.rs::1011] deps: HStore {
    map: {
        XvcEntity(
            3,
            9022053985516504033,
        ): SqliteQueryDigest(
            SqliteQueryDep {
                path: XvcPath(
                    "people.db",
                ),
                query: "SELECT count(*) FROM People;",
                query_digest: None,
                xvc_metadata: None,
            },
        ),
    },
}
[TRACE][pipeline/src/pipeline/deps/compare.rs::449] &stored: SqliteQueryDigest(
    SqliteQueryDep {
        path: XvcPath(
            "people.db",
        ),
        query: "SELECT count(*) FROM People;",
        query_digest: None,
        xvc_metadata: None,
    },
)
[TRACE][core/src/util/pmp.rs::145] path: XvcPath(
    "people.db",
)
[TRACE][core/src/util/pmp.rs::171] xvc_path: XvcPath(
    "people.db",
)
[TRACE][core/src/util/pmp.rs::173] path: AbsolutePath(
    "[CWD]/people.db",
)
[TRACE][core/src/util/pmp.rs::175] &md: Ok(
    Metadata {
        file_type: FileType(
            FileType {
                mode: 33188,
            },
        ),
        is_dir: false,
        is_file: true,
        permissions: Permissions(
            FilePermissions {
                mode: 33188,
            },
        ),
        modified: Ok(
            SystemTime {
                tv_sec: 1722024927,
                tv_nsec: 262703931,
            },
        ),
        accessed: Ok(
            SystemTime {
                tv_sec: 1722025051,
                tv_nsec: 673004106,
            },
        ),
        created: Ok(
            SystemTime {
                tv_sec: 1722024927,
                tv_nsec: 261929105,
            },
        ),
        ..
    },
)
[TRACE][core/src/util/pmp.rs::152] &md: Some(
    XvcMetadata {
        file_type: File,
        size: Some(
            8192,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1722024927,
                tv_nsec: 262703931,
            },
        ),
    },
)
[TRACE][pipeline/src/pipeline/mod.rs::1028] step_dependency_diffs: HStore {
    map: {
        XvcEntity(
            3,
            9022053985516504033,
        ): RecordMissing {
            actual: SqliteQueryDigest(
                SqliteQueryDep {
                    path: XvcPath(
                        "people.db",
                    ),
                    query: "SELECT count(*) FROM People;",
                    query_digest: None,
                    xvc_metadata: Some(
                        XvcMetadata {
                            file_type: File,
                            size: Some(
                                8192,
                            ),
                            modified: Some(
                                SystemTime {
                                    tv_sec: 1722024927,
                                    tv_nsec: 262703931,
                                },
                            ),
                        },
                    ),
                },
            ),
        },
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::1034] diff: RecordMissing {
    actual: SqliteQueryDigest(
        SqliteQueryDep {
            path: XvcPath(
                "people.db",
            ),
            query: "SELECT count(*) FROM People;",
            query_digest: None,
            xvc_metadata: Some(
                XvcMetadata {
                    file_type: File,
                    size: Some(
                        8192,
                    ),
                    modified: Some(
                        SystemTime {
                            tv_sec: 1722024927,
                            tv_nsec: 262703931,
                        },
                    ),
                },
            ),
        },
    ),
}
[TRACE][pipeline/src/pipeline/mod.rs::1035] diff.changed(): true
[TRACE][pipeline/src/pipeline/mod.rs::1040] changed: true
[TRACE][pipeline/src/pipeline/mod.rs::770] step.name: "average-age"
[TRACE][pipeline/src/pipeline/mod.rs::771] &r_next_state: CheckingThoroughDiffs(
    FromSuperficialDiffsChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::773] &step_state: CheckingThoroughDiffs(
    FromSuperficialDiffsChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::660] &step_state: CheckingThoroughDiffs(
    FromSuperficialDiffsChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::1065] deps: HStore {
    map: {
        XvcEntity(
            3,
            9022053985516504033,
        ): SqliteQueryDigest(
            SqliteQueryDep {
                path: XvcPath(
                    "people.db",
                ),
                query: "SELECT count(*) FROM People;",
                query_digest: None,
                xvc_metadata: None,
            },
        ),
    },
}
[TRACE][core/src/util/pmp.rs::145] path: XvcPath(
    "people.db",
)
[TRACE][core/src/util/pmp.rs::152] &md: Some(
    XvcMetadata {
        file_type: File,
        size: Some(
            8192,
        ),
        modified: Some(
            SystemTime {
                tv_sec: 1722024927,
                tv_nsec: 262703931,
            },
        ),
    },
)
[TRACE][pipeline/src/pipeline/deps/sqlite_query.rs::71] col: Integer(
    18,
)
[TRACE][pipeline/src/pipeline/mod.rs::770] step.name: "average-age"
[TRACE][pipeline/src/pipeline/mod.rs::771] &r_next_state: ComparingDiffsAndOutputs(
    FromThoroughDiffsChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::773] &step_state: ComparingDiffsAndOutputs(
    FromThoroughDiffsChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::660] &step_state: ComparingDiffsAndOutputs(
    FromThoroughDiffsChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::770] step.name: "average-age"
[TRACE][pipeline/src/pipeline/mod.rs::771] &r_next_state: WaitingToRun(
    FromDiffsHasChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::773] &step_state: WaitingToRun(
    FromDiffsHasChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::660] &step_state: WaitingToRun(
    FromDiffsHasChanged,
)
[INFO] [average-age] Dependencies has changed
[TRACE][pipeline/src/pipeline/mod.rs::1539] params: StepStateParams {
    xvc_root: XvcRootInner {
        absolute_path: AbsolutePath(
            "[CWD]",
        ),
        xvc_dir: AbsolutePath(
            "[CWD]/.xvc",
        ),
        store_dir: AbsolutePath(
            "[CWD]/.xvc/store",
        ),
        config: XvcConfig {
            current_dir: XvcConfigOption {
                source: Runtime,
                option: AbsolutePath(
                    "[CWD]",
                ),
            },
            config_maps: [
                XvcConfigMap {
                    source: Default,
                    map: {
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "core.verbosity": String(
                            "error",
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "file.list.sort": String(
                            "name-desc",
                        ),
                        "git.use_git": Boolean(
                            true,
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "core.guid": String(
                            "919ce8649f3e32f7",
                        ),
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "pipeline.process_pool_size": Integer(
                            4,
                        ),
                        "git.command": String(
                            "git",
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "file.list.show_dot_files": Boolean(
                            false,
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                        "file.list.no_summary": Boolean(
                            false,
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                    },
                },
                XvcConfigMap {
                    source: Project,
                    map: {
                        "core.guid": String(
                            "59a2d95849757118",
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "git.command": String(
                            "git",
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "git.use_git": Boolean(
                            true,
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "file.list.no_summary": Boolean(
                            false,
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "file.list.show_dot_files": Boolean(
                            false,
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "file.list.sort": String(
                            "name-desc",
                        ),
                        "pipeline.process_pool_size": Integer(
                            4,
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "core.verbosity": String(
                            "error",
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                    },
                },
                XvcConfigMap {
                    source: Local,
                    map: {},
                },
                XvcConfigMap {
                    source: Environment,
                    map: {
                        "TRYCMD_TESTS": String(
                            "storage,file,pipeline,core",
                        ),
                    },
                },
                XvcConfigMap {
                    source: CommandLine,
                    map: {
                        "core.quiet": Boolean(
                            false,
                        ),
                        "core.verbosity": String(
                            "debug",
                        ),
                    },
                },
            ],
            the_config: {
                "pipeline.default_params_file": XvcConfigValue {
                    source: Project,
                    value: String(
                        "params.yaml",
                    ),
                },
                "file.list.sort": XvcConfigValue {
                    source: Project,
                    value: String(
                        "name-desc",
                    ),
                },
                "git.use_git": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        true,
                    ),
                },
                "git.auto_commit": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        true,
                    ),
                },
                "core.verbosity": XvcConfigValue {
                    source: CommandLine,
                    value: String(
                        "debug",
                    ),
                },
                "git.auto_stage": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "file.carry-in.force": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "pipeline.current_pipeline": XvcConfigValue {
                    source: Project,
                    value: String(
                        "default",
                    ),
                },
                "cache.algorithm": XvcConfigValue {
                    source: Project,
                    value: String(
                        "blake3",
                    ),
                },
                "pipeline.process_pool_size": XvcConfigValue {
                    source: Project,
                    value: Integer(
                        4,
                    ),
                },
                "file.list.no_summary": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "file.track.no_parallel": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "file.recheck.method": XvcConfigValue {
                    source: Project,
                    value: String(
                        "copy",
                    ),
                },
                "core.quiet": XvcConfigValue {
                    source: CommandLine,
                    value: Boolean(
                        false,
                    ),
                },
                "file.carry-in.no_parallel": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "core.guid": XvcConfigValue {
                    source: Project,
                    value: String(
                        "59a2d95849757118",
                    ),
                },
                "pipeline.default": XvcConfigValue {
                    source: Project,
                    value: String(
                        "default",
                    ),
                },
                "git.command": XvcConfigValue {
                    source: Project,
                    value: String(
                        "git",
                    ),
                },
                "file.track.force": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "file.track.no_commit": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "file.list.recursive": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "file.list.format": XvcConfigValue {
                    source: Project,
                    value: String(
                        "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                    ),
                },
                "file.track.text_or_binary": XvcConfigValue {
                    source: Project,
                    value: String(
                        "auto",
                    ),
                },
                "TRYCMD_TESTS": XvcConfigValue {
                    source: Environment,
                    value: String(
                        "storage,file,pipeline,core",
                    ),
                },
                "file.list.show_dot_files": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
            },
            init_params: XvcConfigParams {
                default_configuration: "
[core]
# The repository id. Please do not delete or change it.
# This is used to identify the repository and generate paths in storages.
# In the future it may be used to in other ways.
guid = /"919ce8649f3e32f7/"
# Default verbosity level.
# One of /"error/", /"warn/", /"info/"
verbosity = /"error/"

[git]
# Automate git operations.
# Turning this off leads Xvc to behave as if it's not in a Git repository.
# Not recommended unless you're really not using Git
use_git = true
# Command to run Git process.
# You can set this to an absolute path to specify an executable
# If set to a non-absolute path, the executable will be searched in $PATH.
command = /"git/"

# Commit changes in .xvc/ directory after commands.
# You can set this to false if you want to commit manually.
auto_commit = true

# Stage changes in .xvc/ directory without committing.
# auto_commit implies auto_stage.
# If you want to commit manually but don't want to stage after individual Xvc commands, you can set this to true.
auto_stage = false

[cache]
# The hash algorithm used for the cache.
# It may take blake3, blake2, sha2 or sha3 as values.
# All algorithms are selected to produce 256-bit hashes, so sha2 means SHA2-256, blake2 means BLAKE2s, etc.
# The cache path is produced by prepending algorithm name to the cache.
# Blake3 files are in .xvc/b3/, while sha2 files are in .xvc/s2/ etc.
algorithm = /"blake3/"

[file]

[file.track]

# Don't move file content to cache after xvc file track
no_commit = false
# Force to track files even if they are already tracked.
force = false

# Xvc calculates file content digest differently for text and binary files.
# This option controls whether to treat files as text or binary.
# It may take auto, text or binary as values.
# Auto check each file individually and treat it as text if it's text.
text_or_binary = /"auto/"

# Don't use parallelism in track operations.
# Note that some of the operations are implemented in parallel by default, and this option affects some heavier operations.
no_parallel = false

[file.list]

# Format for `xvc file list` rows. You can reorder or remove columns.
# The following are the keys for each row:
# - {acd64}:  actual content digest. All 64 digits from the workspace file's content.
# - {acd8}:  actual content digest. First 8 digits the file content digest.
# - {aft}:  actual file type. Whether the entry is a file (F), directory (D),
#   symlink (S), hardlink (H) or reflink (R).
# - {asz}:  actual size. The size of the workspace file in bytes. It uses MB,
#   GB and TB to represent sizes larger than 1MB.
# - {ats}:  actual timestamp. The timestamp of the workspace file.
# - {cst}:  cache status. One of /"=/", /">/", /"</", /"X/", or /"?/" to show
#   whether the file timestamp is the same as the cached timestamp, newer,
#   older, not cached or not tracked.
# - {name}: The name of the file or directory.
# - {rcd64}:  recorded content digest. All 64 digits.
# - {rcd8}:  recorded content digest. First 8 digits.
# - {rrm}:  recorded recheck method. Whether the entry is linked to the workspace
#   as a copy (C), symlink (S), hardlink (H) or reflink (R).
# - {rsz}:  recorded size. The size of the cached content in bytes. It uses
#   MB, GB and TB to represent sizes larged than 1MB.
# - {rts}:  recorded timestamp. The timestamp of the cached content.
#
# There are no escape sequences in the format string.
# If you want to add a tab, type it to the string.
# If you want to add a literal double curly brace, open an issue.
format = /"{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}/"

# Default sort order for `xvc file list`.
# Valid values are
# none, name-asc, name-desc, size-asc, size-desc, ts-asc, ts-desc.
sort = /"name-desc/"

# Show dot files like .gitignore
show_dot_files = false

# Do not show a summary for as the final row for `xvc file list`.
no_summary = false

# List files recursively always.
recursive = false

[file.carry-in]
# Carry-in the files to cache always, even if they are already present.
force = false

# Don't use parallel move/copy in carry-in
no_parallel = false

[file.recheck]
# The recheck method for Xvc. It may take copy, hardlink, symlink, reflink as values.
# The default is copy to make sure the options is portable.
# Copy duplicates the file content, while hardlink, symlink and reflink only create a new path to the file.
# Note that hardlink and symlink are read-only as they link the files in cache.
method = /"copy/"

[pipeline]
# Name of the current pipeline to run
current_pipeline = /"default/"
# Name of the default pipeline
default = /"default/"
# Name of the default params file name
default_params_file = /"params.yaml/"
# Number of command processes to run concurrently
process_pool_size = 4
#

",
                current_dir: AbsolutePath(
                    "[CWD]",
                ),
                include_system_config: true,
                include_user_config: true,
                project_config_path: Some(
                    AbsolutePath(
                        "[CWD]/.xvc/config.toml",
                    ),
                ),
                local_config_path: Some(
                    AbsolutePath(
                        "[CWD]/.xvc/config.local.toml",
                    ),
                ),
                include_environment_config: true,
                command_line_config: Some(
                    [
                        "core.verbosity = debug",
                        "core.quiet = false",
                    ],
                ),
            },
        },
        local_config_path: AbsolutePath(
            "[CWD]/.xvc/config.local.toml",
        ),
        project_config_path: AbsolutePath(
            "[CWD]/.xvc/config.toml",
        ),
        entity_generator: XvcEntityGenerator {
            counter: 4,
            random: 13513277701724141613,
            dirty: false,
        },
    },
    output_snd: Sender { .. },
    pmp: XvcPathMetadataProvider {
        xvc_root: XvcRootInner {
            absolute_path: AbsolutePath(
                "[CWD]",
            ),
            xvc_dir: AbsolutePath(
                "[CWD]/.xvc",
            ),
            store_dir: AbsolutePath(
                "[CWD]/.xvc/store",
            ),
            config: XvcConfig {
                current_dir: XvcConfigOption {
                    source: Runtime,
                    option: AbsolutePath(
                        "[CWD]",
                    ),
                },
                config_maps: [
                    XvcConfigMap {
                        source: Default,
                        map: {
                            "pipeline.default_params_file": String(
                                "params.yaml",
                            ),
                            "file.carry-in.no_parallel": Boolean(
                                false,
                            ),
                            "file.carry-in.force": Boolean(
                                false,
                            ),
                            "core.verbosity": String(
                                "error",
                            ),
                            "file.recheck.method": String(
                                "copy",
                            ),
                            "file.list.sort": String(
                                "name-desc",
                            ),
                            "git.use_git": Boolean(
                                true,
                            ),
                            "pipeline.current_pipeline": String(
                                "default",
                            ),
                            "core.guid": String(
                                "919ce8649f3e32f7",
                            ),
                            "git.auto_commit": Boolean(
                                true,
                            ),
                            "pipeline.default": String(
                                "default",
                            ),
                            "pipeline.process_pool_size": Integer(
                                4,
                            ),
                            "git.command": String(
                                "git",
                            ),
                            "file.track.no_commit": Boolean(
                                false,
                            ),
                            "file.track.force": Boolean(
                                false,
                            ),
                            "file.list.show_dot_files": Boolean(
                                false,
                            ),
                            "file.list.recursive": Boolean(
                                false,
                            ),
                            "file.list.format": String(
                                "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                            ),
                            "cache.algorithm": String(
                                "blake3",
                            ),
                            "git.auto_stage": Boolean(
                                false,
                            ),
                            "file.list.no_summary": Boolean(
                                false,
                            ),
                            "file.track.text_or_binary": String(
                                "auto",
                            ),
                            "file.track.no_parallel": Boolean(
                                false,
                            ),
                        },
                    },
                    XvcConfigMap {
                        source: Project,
                        map: {
                            "core.guid": String(
                                "59a2d95849757118",
                            ),
                            "cache.algorithm": String(
                                "blake3",
                            ),
                            "file.list.recursive": Boolean(
                                false,
                            ),
                            "git.command": String(
                                "git",
                            ),
                            "pipeline.default_params_file": String(
                                "params.yaml",
                            ),
                            "file.track.force": Boolean(
                                false,
                            ),
                            "file.track.no_parallel": Boolean(
                                false,
                            ),
                            "git.use_git": Boolean(
                                true,
                            ),
                            "file.track.no_commit": Boolean(
                                false,
                            ),
                            "file.recheck.method": String(
                                "copy",
                            ),
                            "file.list.no_summary": Boolean(
                                false,
                            ),
                            "pipeline.current_pipeline": String(
                                "default",
                            ),
                            "git.auto_stage": Boolean(
                                false,
                            ),
                            "git.auto_commit": Boolean(
                                true,
                            ),
                            "file.list.show_dot_files": Boolean(
                                false,
                            ),
                            "file.list.format": String(
                                "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                            ),
                            "file.list.sort": String(
                                "name-desc",
                            ),
                            "pipeline.process_pool_size": Integer(
                                4,
                            ),
                            "file.carry-in.no_parallel": Boolean(
                                false,
                            ),
                            "pipeline.default": String(
                                "default",
                            ),
                            "file.carry-in.force": Boolean(
                                false,
                            ),
                            "core.verbosity": String(
                                "error",
                            ),
                            "file.track.text_or_binary": String(
                                "auto",
                            ),
                        },
                    },
                    XvcConfigMap {
                        source: Local,
                        map: {},
                    },
                    XvcConfigMap {
                        source: Environment,
                        map: {
                            "TRYCMD_TESTS": String(
                                "storage,file,pipeline,core",
                            ),
                        },
                    },
                    XvcConfigMap {
                        source: CommandLine,
                        map: {
                            "core.quiet": Boolean(
                                false,
                            ),
                            "core.verbosity": String(
                                "debug",
                            ),
                        },
                    },
                ],
                the_config: {
                    "pipeline.default_params_file": XvcConfigValue {
                        source: Project,
                        value: String(
                            "params.yaml",
                        ),
                    },
                    "file.list.sort": XvcConfigValue {
                        source: Project,
                        value: String(
                            "name-desc",
                        ),
                    },
                    "git.use_git": XvcConfigValue {
                        source: Project,
                        value: Boolean(
                            true,
                        ),
                    },
                    "git.auto_commit": XvcConfigValue {
                        source: Project,
                        value: Boolean(
                            true,
                        ),
                    },
                    "core.verbosity": XvcConfigValue {
                        source: CommandLine,
                        value: String(
                            "debug",
                        ),
                    },
                    "git.auto_stage": XvcConfigValue {
                        source: Project,
                        value: Boolean(
                            false,
                        ),
                    },
                    "file.carry-in.force": XvcConfigValue {
                        source: Project,
                        value: Boolean(
                            false,
                        ),
                    },
                    "pipeline.current_pipeline": XvcConfigValue {
                        source: Project,
                        value: String(
                            "default",
                        ),
                    },
                    "cache.algorithm": XvcConfigValue {
                        source: Project,
                        value: String(
                            "blake3",
                        ),
                    },
                    "pipeline.process_pool_size": XvcConfigValue {
                        source: Project,
                        value: Integer(
                            4,
                        ),
                    },
                    "file.list.no_summary": XvcConfigValue {
                        source: Project,
                        value: Boolean(
                            false,
                        ),
                    },
                    "file.track.no_parallel": XvcConfigValue {
                        source: Project,
                        value: Boolean(
                            false,
                        ),
                    },
                    "file.recheck.method": XvcConfigValue {
                        source: Project,
                        value: String(
                            "copy",
                        ),
                    },
                    "core.quiet": XvcConfigValue {
                        source: CommandLine,
                        value: Boolean(
                            false,
                        ),
                    },
                    "file.carry-in.no_parallel": XvcConfigValue {
                        source: Project,
                        value: Boolean(
                            false,
                        ),
                    },
                    "core.guid": XvcConfigValue {
                        source: Project,
                        value: String(
                            "59a2d95849757118",
                        ),
                    },
                    "pipeline.default": XvcConfigValue {
                        source: Project,
                        value: String(
                            "default",
                        ),
                    },
                    "git.command": XvcConfigValue {
                        source: Project,
                        value: String(
                            "git",
                        ),
                    },
                    "file.track.force": XvcConfigValue {
                        source: Project,
                        value: Boolean(
                            false,
                        ),
                    },
                    "file.track.no_commit": XvcConfigValue {
                        source: Project,
                        value: Boolean(
                            false,
                        ),
                    },
                    "file.list.recursive": XvcConfigValue {
                        source: Project,
                        value: Boolean(
                            false,
                        ),
                    },
                    "file.list.format": XvcConfigValue {
                        source: Project,
                        value: String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                    },
                    "file.track.text_or_binary": XvcConfigValue {
                        source: Project,
                        value: String(
                            "auto",
                        ),
                    },
                    "TRYCMD_TESTS": XvcConfigValue {
                        source: Environment,
                        value: String(
                            "storage,file,pipeline,core",
                        ),
                    },
                    "file.list.show_dot_files": XvcConfigValue {
                        source: Project,
                        value: Boolean(
                            false,
                        ),
                    },
                },
                init_params: XvcConfigParams {
                    default_configuration: "
[core]
# The repository id. Please do not delete or change it.
# This is used to identify the repository and generate paths in storages.
# In the future it may be used to in other ways.
guid = /"919ce8649f3e32f7/"
# Default verbosity level.
# One of /"error/", /"warn/", /"info/"
verbosity = /"error/"

[git]
# Automate git operations.
# Turning this off leads Xvc to behave as if it's not in a Git repository.
# Not recommended unless you're really not using Git
use_git = true
# Command to run Git process.
# You can set this to an absolute path to specify an executable
# If set to a non-absolute path, the executable will be searched in $PATH.
command = /"git/"

# Commit changes in .xvc/ directory after commands.
# You can set this to false if you want to commit manually.
auto_commit = true

# Stage changes in .xvc/ directory without committing.
# auto_commit implies auto_stage.
# If you want to commit manually but don't want to stage after individual Xvc commands, you can set this to true.
auto_stage = false

[cache]
# The hash algorithm used for the cache.
# It may take blake3, blake2, sha2 or sha3 as values.
# All algorithms are selected to produce 256-bit hashes, so sha2 means SHA2-256, blake2 means BLAKE2s, etc.
# The cache path is produced by prepending algorithm name to the cache.
# Blake3 files are in .xvc/b3/, while sha2 files are in .xvc/s2/ etc.
algorithm = /"blake3/"

[file]

[file.track]

# Don't move file content to cache after xvc file track
no_commit = false
# Force to track files even if they are already tracked.
force = false

# Xvc calculates file content digest differently for text and binary files.
# This option controls whether to treat files as text or binary.
# It may take auto, text or binary as values.
# Auto check each file individually and treat it as text if it's text.
text_or_binary = /"auto/"

# Don't use parallelism in track operations.
# Note that some of the operations are implemented in parallel by default, and this option affects some heavier operations.
no_parallel = false

[file.list]

# Format for `xvc file list` rows. You can reorder or remove columns.
# The following are the keys for each row:
# - {acd64}:  actual content digest. All 64 digits from the workspace file's content.
# - {acd8}:  actual content digest. First 8 digits the file content digest.
# - {aft}:  actual file type. Whether the entry is a file (F), directory (D),
#   symlink (S), hardlink (H) or reflink (R).
# - {asz}:  actual size. The size of the workspace file in bytes. It uses MB,
#   GB and TB to represent sizes larger than 1MB.
# - {ats}:  actual timestamp. The timestamp of the workspace file.
# - {cst}:  cache status. One of /"=/", /">/", /"</", /"X/", or /"?/" to show
#   whether the file timestamp is the same as the cached timestamp, newer,
#   older, not cached or not tracked.
# - {name}: The name of the file or directory.
# - {rcd64}:  recorded content digest. All 64 digits.
# - {rcd8}:  recorded content digest. First 8 digits.
# - {rrm}:  recorded recheck method. Whether the entry is linked to the workspace
#   as a copy (C), symlink (S), hardlink (H) or reflink (R).
# - {rsz}:  recorded size. The size of the cached content in bytes. It uses
#   MB, GB and TB to represent sizes larged than 1MB.
# - {rts}:  recorded timestamp. The timestamp of the cached content.
#
# There are no escape sequences in the format string.
# If you want to add a tab, type it to the string.
# If you want to add a literal double curly brace, open an issue.
format = /"{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}/"

# Default sort order for `xvc file list`.
# Valid values are
# none, name-asc, name-desc, size-asc, size-desc, ts-asc, ts-desc.
sort = /"name-desc/"

# Show dot files like .gitignore
show_dot_files = false

# Do not show a summary for as the final row for `xvc file list`.
no_summary = false

# List files recursively always.
recursive = false

[file.carry-in]
# Carry-in the files to cache always, even if they are already present.
force = false

# Don't use parallel move/copy in carry-in
no_parallel = false

[file.recheck]
# The recheck method for Xvc. It may take copy, hardlink, symlink, reflink as values.
# The default is copy to make sure the options is portable.
# Copy duplicates the file content, while hardlink, symlink and reflink only create a new path to the file.
# Note that hardlink and symlink are read-only as they link the files in cache.
method = /"copy/"

[pipeline]
# Name of the current pipeline to run
current_pipeline = /"default/"
# Name of the default pipeline
default = /"default/"
# Name of the default params file name
default_params_file = /"params.yaml/"
# Number of command processes to run concurrently
process_pool_size = 4
#

",
                    current_dir: AbsolutePath(
                        "[CWD]",
                    ),
                    include_system_config: true,
                    include_user_config: true,
                    project_config_path: Some(
                        AbsolutePath(
                            "[CWD]/.xvc/config.toml",
                        ),
                    ),
                    local_config_path: Some(
                        AbsolutePath(
                            "[CWD]/.xvc/config.local.toml",
                        ),
                    ),
                    include_environment_config: true,
                    command_line_config: Some(
                        [
                            "core.verbosity = debug",
                            "core.quiet = false",
                        ],
                    ),
                },
            },
            local_config_path: AbsolutePath(
                "[CWD]/.xvc/config.local.toml",
            ),
            project_config_path: AbsolutePath(
                "[CWD]/.xvc/config.toml",
            ),
            entity_generator: XvcEntityGenerator {
                counter: 4,
                random: 13513277701724141613,
                dirty: false,
            },
        },
        path_map: RwLock {
            data: {
                XvcPath(
                    "people.db",
                ): XvcMetadata {
                    file_type: File,
                    size: Some(
                        8192,
                    ),
                    modified: Some(
                        SystemTime {
                            tv_sec: 1722024927,
                            tv_nsec: 262703931,
                        },
                    ),
                },
            },
            poisoned: false,
            ..
        },
        kill_signal_sender: Sender { .. },
        background_thread: Mutex {
            data: JoinHandle { .. },
            poisoned: false,
            ..
        },
        output_sender: Sender { .. },
        ignore_rules: IgnoreRules {
            root: "[CWD]",
            ignore_patterns: RwLock {
                data: [
                    Pattern {
                        pattern: Glob {
                            glob: "**/.xvc",
                            re: "(?-u)^(?:/?|.*/)//.xvc$",
                            opts: GlobOptions {
                                case_insensitive: false,
                                literal_separator: false,
                                backslash_escape: true,
                                empty_alternates: false,
                            },
                            tokens: Tokens(
                                [
                                    RecursivePrefix,
                                    Literal(
                                        '.',
                                    ),
                                    Literal(
                                        'x',
                                    ),
                                    Literal(
                                        'v',
                                    ),
                                    Literal(
                                        'c',
                                    ),
                                ],
                            ),
                        },
                        original: ".xvc",
                        source: Global,
                        effect: Ignore,
                        relativity: Anywhere,
                        path_kind: Any,
                    },
                    Pattern {
                        pattern: Glob {
                            glob: "**/.git",
                            re: "(?-u)^(?:/?|.*/)//.git$",
                            opts: GlobOptions {
                                case_insensitive: false,
                                literal_separator: false,
                                backslash_escape: true,
                                empty_alternates: false,
                            },
                            tokens: Tokens(
                                [
                                    RecursivePrefix,
                                    Literal(
                                        '.',
                                    ),
                                    Literal(
                                        'g',
                                    ),
                                    Literal(
                                        'i',
                                    ),
                                    Literal(
                                        't',
                                    ),
                                ],
                            ),
                        },
                        original: ".git",
                        source: Global,
                        effect: Ignore,
                        relativity: Anywhere,
                        path_kind: Any,
                    },
                    Pattern {
                        pattern: Glob {
                            glob: "**/.DS_Store",
                            re: "(?-u)^(?:/?|.*/)//.DS_Store$",
                            opts: GlobOptions {
                                case_insensitive: false,
                                literal_separator: false,
                                backslash_escape: true,
                                empty_alternates: false,
                            },
                            tokens: Tokens(
                                [
                                    RecursivePrefix,
                                    Literal(
                                        '.',
                                    ),
                                    Literal(
                                        'D',
                                    ),
                                    Literal(
                                        'S',
                                    ),
                                    Literal(
                                        '_',
                                    ),
                                    Literal(
                                        'S',
                                    ),
                                    Literal(
                                        't',
                                    ),
                                    Literal(
                                        'o',
                                    ),
                                    Literal(
                                        'r',
                                    ),
                                    Literal(
                                        'e',
                                    ),
                                ],
                            ),
                        },
                        original: ".DS_Store",
                        source: File {
                            path: ".xvcignore",
                            line: 6,
                        },
                        effect: Ignore,
                        relativity: Anywhere,
                        path_kind: Any,
                    },
                ],
                poisoned: false,
                ..
            },
            whitelist_patterns: RwLock {
                data: [],
                poisoned: false,
                ..
            },
            whitelist_set: RwLock {
                data: GlobSet {
                    len: 0,
                    strats: [],
                },
                poisoned: false,
                ..
            },
            ignore_set: RwLock {
                data: GlobSet {
                    len: 3,
                    strats: [
                        Extension(
                            ExtensionStrategy(
                                {},
                            ),
                        ),
                        BasenameLiteral(
                            BasenameLiteralStrategy(
                                {
                                    [
                                        46,
                                        120,
                                        118,
                                        99,
                                    ]: [
                                        0,
                                    ],
                                    [
                                        46,
                                        68,
                                        83,
                                        95,
                                        83,
                                        116,
                                        111,
                                        114,
                                        101,
                                    ]: [
                                        2,
                                    ],
                                    [
                                        46,
                                        103,
                                        105,
                                        116,
                                    ]: [
                                        1,
                                    ],
                                },
                            ),
                        ),
                        Literal(
                            LiteralStrategy(
                                {},
                            ),
                        ),
                        Suffix(
                            SuffixStrategy {
                                matcher: AhoCorasick(
                                    dfa::DFA(
                                    D 000000: /x00 => 0
                                    F 000001:
                                     >000002: /x00 => 2
                                      000003: /x00 => 0
                                    match kind: Standard
                                    prefilter: false
                                    state length: 4
                                    pattern length: 0
                                    shortest pattern length: 18446744073709551615
                                    longest pattern length: 0
                                    alphabet length: 1
                                    stride: 1
                                    byte classes: ByteClasses(0 => [0-255])
                                    memory usage: 16
                                    )
                                    ,
                                ),
                                map: [],
                                longest: 0,
                            },
                        ),
                        Prefix(
                            PrefixStrategy {
                                matcher: AhoCorasick(
                                    dfa::DFA(
                                    D 000000: /x00 => 0
                                    F 000001:
                                     >000002: /x00 => 2
                                      000003: /x00 => 0
                                    match kind: Standard
                                    prefilter: false
                                    state length: 4
                                    pattern length: 0
                                    shortest pattern length: 18446744073709551615
                                    longest pattern length: 0
                                    alphabet length: 1
                                    stride: 1
                                    byte classes: ByteClasses(0 => [0-255])
                                    memory usage: 16
                                    )
                                    ,
                                ),
                                map: [],
                                longest: 0,
                            },
                        ),
                        RequiredExtension(
                            RequiredExtensionStrategy(
                                {},
                            ),
                        ),
                        Regex(
                            RegexSetStrategy {
                                matcher: Regex {
                                    imp: RegexI {
                                        strat: Core {
                                            info: RegexInfo(
                                                RegexInfoI {
                                                    config: Config {
                                                        match_kind: Some(
                                                            All,
                                                        ),
                                                        utf8_empty: Some(
                                                            false,
                                                        ),
                                                        autopre: None,
                                                        pre: None,
                                                        which_captures: None,
                                                        nfa_size_limit: Some(
                                                            Some(
                                                                10485760,
                                                            ),
                                                        ),
                                                        onepass_size_limit: None,
                                                        hybrid_cache_capacity: Some(
                                                            10485760,
                                                        ),
                                                        hybrid: None,
                                                        dfa: None,
                                                        dfa_size_limit: None,
                                                        dfa_state_limit: None,
                                                        onepass: None,
                                                        backtrack: None,
                                                        byte_classes: None,
                                                        line_terminator: None,
                                                    },
                                                    props: [],
                                                    props_union: Properties(
                                                        PropertiesI {
                                                            minimum_len: None,
                                                            maximum_len: None,
                                                            look_set: ∅,
                                                            look_set_prefix: ∅,
                                                            look_set_suffix: ∅,
                                                            look_set_prefix_any: ∅,
                                                            look_set_suffix_any: ∅,
                                                            utf8: true,
                                                            explicit_captures_len: 0,
                                                            static_explicit_captures_len: None,
                                                            literal: false,
                                                            alternation_literal: true,
                                                        },
                                                    ),
                                                },
                                            ),
                                            pre: None,
                                            nfa: thompson::NFA(
                                            ^000000: FAIL

                                            transition equivalence classes: ByteClasses(0 => [/x00-/xFF], 1 => [EOI])
                                            )
                                            ,
                                            nfarev: Some(
                                                thompson::NFA(
                                                ^000000: FAIL

                                                transition equivalence classes: ByteClasses(0 => [/x00-/xFF], 1 => [EOI])
                                                )
                                                ,
                                            ),
                                            pikevm: PikeVM(
                                                PikeVMEngine(
                                                    PikeVM {
                                                        config: Config {
                                                            match_kind: Some(
                                                                All,
                                                            ),
                                                            pre: Some(
                                                                None,
                                                            ),
                                                        },
                                                        nfa: thompson::NFA(
                                                        ^000000: FAIL

                                                        transition equivalence classes: ByteClasses(0 => [/x00-/xFF], 1 => [EOI])
                                                        )
                                                        ,
                                                    },
                                                ),
                                            ),
                                            backtrack: BoundedBacktracker(
                                                None,
                                            ),
                                            onepass: OnePass(
                                                None,
                                            ),
                                            hybrid: Hybrid(
                                                Some(
                                                    HybridEngine(
                                                        Regex {
                                                            forward: DFA {
                                                                config: Config {
                                                                    match_kind: Some(
                                                                        All,
                                                                    ),
                                                                    pre: Some(
                                                                        None,
                                                                    ),
                                                                    starts_for_each_pattern: Some(
                                                                        true,
                                                                    ),
                                                                    byte_classes: Some(
                                                                        true,
                                                                    ),
                                                                    unicode_word_boundary: Some(
                                                                        true,
                                                                    ),
                                                                    quitset: None,
                                                                    specialize_start_states: Some(
                                                                        false,
                                                                    ),
                                                                    cache_capacity: Some(
                                                                        10485760,
                                                                    ),
                                                                    skip_cache_capacity_check: Some(
                                                                        false,
                                                                    ),
                                                                    minimum_cache_clear_count: Some(
                                                                        Some(
                                                                            3,
                                                                        ),
                                                                    ),
                                                                    minimum_bytes_per_state: Some(
                                                                        Some(
                                                                            10,
                                                                        ),
                                                                    ),
                                                                },
                                                                nfa: thompson::NFA(
                                                                ^000000: FAIL

                                                                transition equivalence classes: ByteClasses(0 => [/x00-/xFF], 1 => [EOI])
                                                                )
                                                                ,
                                                                stride2: 1,
                                                                start_map: StartByteMap{/x00 => NonWordByte, /x01 => NonWordByte, /x02 => NonWordByte, /x03 => NonWordByte, /x04 => NonWordByte, /x05 => NonWordByte, /x06 => NonWordByte, /x07 => NonWordByte, /x08 => NonWordByte, /t => NonWordByte,
 => LineLF, /x0B => NonWordByte, /x0C => NonWordByte, /r => LineCR, /x0E => NonWordByte, /x0F => NonWordByte, /x10 => NonWordByte, /x11 => NonWordByte, /x12 => NonWordByte, /x13 => NonWordByte, /x14 => NonWordByte, /x15 => NonWordByte, /x16 => NonWordByte, /x17 => NonWordByte, /x18 => NonWordByte, /x19 => NonWordByte, /x1A => NonWordByte, /x1B => NonWordByte, /x1C => NonWordByte, /x1D => NonWordByte, /x1E => NonWordByte, /x1F => NonWordByte, ' ' => NonWordByte, ! => NonWordByte, /" => NonWordByte, # => NonWordByte, $ => NonWordByte, % => NonWordByte, & => NonWordByte, /' => NonWordByte, ( => NonWordByte, ) => NonWordByte, * => NonWordByte, + => NonWordByte, , => NonWordByte, - => NonWordByte, . => NonWordByte, / => NonWordByte, 0 => WordByte, 1 => WordByte, 2 => WordByte, 3 => WordByte, 4 => WordByte, 5 => WordByte, 6 => WordByte, 7 => WordByte, 8 => WordByte, 9 => WordByte, : => NonWordByte, ; => NonWordByte, < => NonWordByte, = => NonWordByte, > => NonWordByte, ? => NonWordByte, @ => NonWordByte, A => WordByte, B => WordByte, C => WordByte, D => WordByte, E => WordByte, F => WordByte, G => WordByte, H => WordByte, I => WordByte, J => WordByte, K => WordByte, L => WordByte, M => WordByte, N => WordByte, O => WordByte, P => WordByte, Q => WordByte, R => WordByte, S => WordByte, T => WordByte, U => WordByte, V => WordByte, W => WordByte, X => WordByte, Y => WordByte, Z => WordByte, [ => NonWordByte, // => NonWordByte, ] => NonWordByte, ^ => NonWordByte, _ => WordByte, ` => NonWordByte, a => WordByte, b => WordByte, c => WordByte, d => WordByte, e => WordByte, f => WordByte, g => WordByte, h => WordByte, i => WordByte, j => WordByte, k => WordByte, l => WordByte, m => WordByte, n => WordByte, o => WordByte, p => WordByte, q => WordByte, r => WordByte, s => WordByte, t => WordByte, u => WordByte, v => WordByte, w => WordByte, x => WordByte, y => WordByte, z => WordByte, { => NonWordByte, | => NonWordByte, } => NonWordByte, ~ => NonWordByte, /x7F => NonWordByte, /x80 => NonWordByte, /x81 => NonWordByte, /x82 => NonWordByte, /x83 => NonWordByte, /x84 => NonWordByte, /x85 => NonWordByte, /x86 => NonWordByte, /x87 => NonWordByte, /x88 => NonWordByte, /x89 => NonWordByte, /x8A => NonWordByte, /x8B => NonWordByte, /x8C => NonWordByte, /x8D => NonWordByte, /x8E => NonWordByte, /x8F => NonWordByte, /x90 => NonWordByte, /x91 => NonWordByte, /x92 => NonWordByte, /x93 => NonWordByte, /x94 => NonWordByte, /x95 => NonWordByte, /x96 => NonWordByte, /x97 => NonWordByte, /x98 => NonWordByte, /x99 => NonWordByte, /x9A => NonWordByte, /x9B => NonWordByte, /x9C => NonWordByte, /x9D => NonWordByte, /x9E => NonWordByte, /x9F => NonWordByte, /xA0 => NonWordByte, /xA1 => NonWordByte, /xA2 => NonWordByte, /xA3 => NonWordByte, /xA4 => NonWordByte, /xA5 => NonWordByte, /xA6 => NonWordByte, /xA7 => NonWordByte, /xA8 => NonWordByte, /xA9 => NonWordByte, /xAA => NonWordByte, /xAB => NonWordByte, /xAC => NonWordByte, /xAD => NonWordByte, /xAE => NonWordByte, /xAF => NonWordByte, /xB0 => NonWordByte, /xB1 => NonWordByte, /xB2 => NonWordByte, /xB3 => NonWordByte, /xB4 => NonWordByte, /xB5 => NonWordByte, /xB6 => NonWordByte, /xB7 => NonWordByte, /xB8 => NonWordByte, /xB9 => NonWordByte, /xBA => NonWordByte, /xBB => NonWordByte, /xBC => NonWordByte, /xBD => NonWordByte, /xBE => NonWordByte, /xBF => NonWordByte, /xC0 => NonWordByte, /xC1 => NonWordByte, /xC2 => NonWordByte, /xC3 => NonWordByte, /xC4 => NonWordByte, /xC5 => NonWordByte, /xC6 => NonWordByte, /xC7 => NonWordByte, /xC8 => NonWordByte, /xC9 => NonWordByte, /xCA => NonWordByte, /xCB => NonWordByte, /xCC => NonWordByte, /xCD => NonWordByte, /xCE => NonWordByte, /xCF => NonWordByte, /xD0 => NonWordByte, /xD1 => NonWordByte, /xD2 => NonWordByte, /xD3 => NonWordByte, /xD4 => NonWordByte, /xD5 => NonWordByte, /xD6 => NonWordByte, /xD7 => NonWordByte, /xD8 => NonWordByte, /xD9 => NonWordByte, /xDA => NonWordByte, /xDB => NonWordByte, /xDC => NonWordByte, /xDD => NonWordByte, /xDE => NonWordByte, /xDF => NonWordByte, /xE0 => NonWordByte, /xE1 => NonWordByte, /xE2 => NonWordByte, /xE3 => NonWordByte, /xE4 => NonWordByte, /xE5 => NonWordByte, /xE6 => NonWordByte, /xE7 => NonWordByte, /xE8 => NonWordByte, /xE9 => NonWordByte, /xEA => NonWordByte, /xEB => NonWordByte, /xEC => NonWordByte, /xED => NonWordByte, /xEE => NonWordByte, /xEF => NonWordByte, /xF0 => NonWordByte, /xF1 => NonWordByte, /xF2 => NonWordByte, /xF3 => NonWordByte, /xF4 => NonWordByte, /xF5 => NonWordByte, /xF6 => NonWordByte, /xF7 => NonWordByte, /xF8 => NonWordByte, /xF9 => NonWordByte, /xFA => NonWordByte, /xFB => NonWordByte, /xFC => NonWordByte, /xFD => NonWordByte, /xFE => NonWordByte, /xFF => NonWordByte},
                                                                classes: ByteClasses(0 => [/x00-/xFF], 1 => [EOI]),
                                                                quitset: ByteSet {
                                                                    bits: {},
                                                                },
                                                                cache_capacity: 10485760,
                                                            },
                                                            reverse: DFA {
                                                                config: Config {
                                                                    match_kind: Some(
                                                                        All,
                                                                    ),
                                                                    pre: Some(
                                                                        None,
                                                                    ),
                                                                    starts_for_each_pattern: Some(
                                                                        true,
                                                                    ),
                                                                    byte_classes: Some(
                                                                        true,
                                                                    ),
                                                                    unicode_word_boundary: Some(
                                                                        true,
                                                                    ),
                                                                    quitset: None,
                                                                    specialize_start_states: Some(
                                                                        false,
                                                                    ),
                                                                    cache_capacity: Some(
                                                                        10485760,
                                                                    ),
                                                                    skip_cache_capacity_check: Some(
                                                                        false,
                                                                    ),
                                                                    minimum_cache_clear_count: Some(
                                                                        Some(
                                                                            3,
                                                                        ),
                                                                    ),
                                                                    minimum_bytes_per_state: Some(
                                                                        Some(
                                                                            10,
                                                                        ),
                                                                    ),
                                                                },
                                                                nfa: thompson::NFA(
                                                                ^000000: FAIL

                                                                transition equivalence classes: ByteClasses(0 => [/x00-/xFF], 1 => [EOI])
                                                                )
                                                                ,
                                                                stride2: 1,
                                                                start_map: StartByteMap{/x00 => NonWordByte, /x01 => NonWordByte, /x02 => NonWordByte, /x03 => NonWordByte, /x04 => NonWordByte, /x05 => NonWordByte, /x06 => NonWordByte, /x07 => NonWordByte, /x08 => NonWordByte, /t => NonWordByte,
 => LineLF, /x0B => NonWordByte, /x0C => NonWordByte, /r => LineCR, /x0E => NonWordByte, /x0F => NonWordByte, /x10 => NonWordByte, /x11 => NonWordByte, /x12 => NonWordByte, /x13 => NonWordByte, /x14 => NonWordByte, /x15 => NonWordByte, /x16 => NonWordByte, /x17 => NonWordByte, /x18 => NonWordByte, /x19 => NonWordByte, /x1A => NonWordByte, /x1B => NonWordByte, /x1C => NonWordByte, /x1D => NonWordByte, /x1E => NonWordByte, /x1F => NonWordByte, ' ' => NonWordByte, ! => NonWordByte, /" => NonWordByte, # => NonWordByte, $ => NonWordByte, % => NonWordByte, & => NonWordByte, /' => NonWordByte, ( => NonWordByte, ) => NonWordByte, * => NonWordByte, + => NonWordByte, , => NonWordByte, - => NonWordByte, . => NonWordByte, / => NonWordByte, 0 => WordByte, 1 => WordByte, 2 => WordByte, 3 => WordByte, 4 => WordByte, 5 => WordByte, 6 => WordByte, 7 => WordByte, 8 => WordByte, 9 => WordByte, : => NonWordByte, ; => NonWordByte, < => NonWordByte, = => NonWordByte, > => NonWordByte, ? => NonWordByte, @ => NonWordByte, A => WordByte, B => WordByte, C => WordByte, D => WordByte, E => WordByte, F => WordByte, G => WordByte, H => WordByte, I => WordByte, J => WordByte, K => WordByte, L => WordByte, M => WordByte, N => WordByte, O => WordByte, P => WordByte, Q => WordByte, R => WordByte, S => WordByte, T => WordByte, U => WordByte, V => WordByte, W => WordByte, X => WordByte, Y => WordByte, Z => WordByte, [ => NonWordByte, // => NonWordByte, ] => NonWordByte, ^ => NonWordByte, _ => WordByte, ` => NonWordByte, a => WordByte, b => WordByte, c => WordByte, d => WordByte, e => WordByte, f => WordByte, g => WordByte, h => WordByte, i => WordByte, j => WordByte, k => WordByte, l => WordByte, m => WordByte, n => WordByte, o => WordByte, p => WordByte, q => WordByte, r => WordByte, s => WordByte, t => WordByte, u => WordByte, v => WordByte, w => WordByte, x => WordByte, y => WordByte, z => WordByte, { => NonWordByte, | => NonWordByte, } => NonWordByte, ~ => NonWordByte, /x7F => NonWordByte, /x80 => NonWordByte, /x81 => NonWordByte, /x82 => NonWordByte, /x83 => NonWordByte, /x84 => NonWordByte, /x85 => NonWordByte, /x86 => NonWordByte, /x87 => NonWordByte, /x88 => NonWordByte, /x89 => NonWordByte, /x8A => NonWordByte, /x8B => NonWordByte, /x8C => NonWordByte, /x8D => NonWordByte, /x8E => NonWordByte, /x8F => NonWordByte, /x90 => NonWordByte, /x91 => NonWordByte, /x92 => NonWordByte, /x93 => NonWordByte, /x94 => NonWordByte, /x95 => NonWordByte, /x96 => NonWordByte, /x97 => NonWordByte, /x98 => NonWordByte, /x99 => NonWordByte, /x9A => NonWordByte, /x9B => NonWordByte, /x9C => NonWordByte, /x9D => NonWordByte, /x9E => NonWordByte, /x9F => NonWordByte, /xA0 => NonWordByte, /xA1 => NonWordByte, /xA2 => NonWordByte, /xA3 => NonWordByte, /xA4 => NonWordByte, /xA5 => NonWordByte, /xA6 => NonWordByte, /xA7 => NonWordByte, /xA8 => NonWordByte, /xA9 => NonWordByte, /xAA => NonWordByte, /xAB => NonWordByte, /xAC => NonWordByte, /xAD => NonWordByte, /xAE => NonWordByte, /xAF => NonWordByte, /xB0 => NonWordByte, /xB1 => NonWordByte, /xB2 => NonWordByte, /xB3 => NonWordByte, /xB4 => NonWordByte, /xB5 => NonWordByte, /xB6 => NonWordByte, /xB7 => NonWordByte, /xB8 => NonWordByte, /xB9 => NonWordByte, /xBA => NonWordByte, /xBB => NonWordByte, /xBC => NonWordByte, /xBD => NonWordByte, /xBE => NonWordByte, /xBF => NonWordByte, /xC0 => NonWordByte, /xC1 => NonWordByte, /xC2 => NonWordByte, /xC3 => NonWordByte, /xC4 => NonWordByte, /xC5 => NonWordByte, /xC6 => NonWordByte, /xC7 => NonWordByte, /xC8 => NonWordByte, /xC9 => NonWordByte, /xCA => NonWordByte, /xCB => NonWordByte, /xCC => NonWordByte, /xCD => NonWordByte, /xCE => NonWordByte, /xCF => NonWordByte, /xD0 => NonWordByte, /xD1 => NonWordByte, /xD2 => NonWordByte, /xD3 => NonWordByte, /xD4 => NonWordByte, /xD5 => NonWordByte, /xD6 => NonWordByte, /xD7 => NonWordByte, /xD8 => NonWordByte, /xD9 => NonWordByte, /xDA => NonWordByte, /xDB => NonWordByte, /xDC => NonWordByte, /xDD => NonWordByte, /xDE => NonWordByte, /xDF => NonWordByte, /xE0 => NonWordByte, /xE1 => NonWordByte, /xE2 => NonWordByte, /xE3 => NonWordByte, /xE4 => NonWordByte, /xE5 => NonWordByte, /xE6 => NonWordByte, /xE7 => NonWordByte, /xE8 => NonWordByte, /xE9 => NonWordByte, /xEA => NonWordByte, /xEB => NonWordByte, /xEC => NonWordByte, /xED => NonWordByte, /xEE => NonWordByte, /xEF => NonWordByte, /xF0 => NonWordByte, /xF1 => NonWordByte, /xF2 => NonWordByte, /xF3 => NonWordByte, /xF4 => NonWordByte, /xF5 => NonWordByte, /xF6 => NonWordByte, /xF7 => NonWordByte, /xF8 => NonWordByte, /xF9 => NonWordByte, /xFA => NonWordByte, /xFB => NonWordByte, /xFC => NonWordByte, /xFD => NonWordByte, /xFE => NonWordByte, /xFF => NonWordByte},
                                                                classes: ByteClasses(0 => [/x00-/xFF], 1 => [EOI]),
                                                                quitset: ByteSet {
                                                                    bits: {},
                                                                },
                                                                cache_capacity: 10485760,
                                                            },
                                                        },
                                                    ),
                                                ),
                                            ),
                                            dfa: DFA(
                                                None,
                                            ),
                                        },
                                        info: RegexInfo(
                                            RegexInfoI {
                                                config: Config {
                                                    match_kind: Some(
                                                        All,
                                                    ),
                                                    utf8_empty: Some(
                                                        false,
                                                    ),
                                                    autopre: None,
                                                    pre: None,
                                                    which_captures: None,
                                                    nfa_size_limit: Some(
                                                        Some(
                                                            10485760,
                                                        ),
                                                    ),
                                                    onepass_size_limit: None,
                                                    hybrid_cache_capacity: Some(
                                                        10485760,
                                                    ),
                                                    hybrid: None,
                                                    dfa: None,
                                                    dfa_size_limit: None,
                                                    dfa_state_limit: None,
                                                    onepass: None,
                                                    backtrack: None,
                                                    byte_classes: None,
                                                    line_terminator: None,
                                                },
                                                props: [],
                                                props_union: Properties(
                                                    PropertiesI {
                                                        minimum_len: None,
                                                        maximum_len: None,
                                                        look_set: ∅,
                                                        look_set_prefix: ∅,
                                                        look_set_suffix: ∅,
                                                        look_set_prefix_any: ∅,
                                                        look_set_suffix_any: ∅,
                                                        utf8: true,
                                                        explicit_captures_len: 0,
                                                        static_explicit_captures_len: None,
                                                        literal: false,
                                                        alternation_literal: true,
                                                    },
                                                ),
                                            },
                                        ),
                                    },
                                    pool: Pool(
                                        Pool {
                                            stacks: [
                                                CacheLine(
                                                    Mutex {
                                                        data: [],
                                                        poisoned: false,
                                                        ..
                                                    },
                                                ),
                                                CacheLine(
                                                    Mutex {
                                                        data: [],
                                                        poisoned: false,
                                                        ..
                                                    },
                                                ),
                                                CacheLine(
                                                    Mutex {
                                                        data: [],
                                                        poisoned: false,
                                                        ..
                                                    },
                                                ),
                                                CacheLine(
                                                    Mutex {
                                                        data: [],
                                                        poisoned: false,
                                                        ..
                                                    },
                                                ),
                                                CacheLine(
                                                    Mutex {
                                                        data: [],
                                                        poisoned: false,
                                                        ..
                                                    },
                                                ),
                                                CacheLine(
                                                    Mutex {
                                                        data: [],
                                                        poisoned: false,
                                                        ..
                                                    },
                                                ),
                                                CacheLine(
                                                    Mutex {
                                                        data: [],
                                                        poisoned: false,
                                                        ..
                                                    },
                                                ),
                                                CacheLine(
                                                    Mutex {
                                                        data: [],
                                                        poisoned: false,
                                                        ..
                                                    },
                                                ),
                                            ],
                                            owner: 0,
                                            owner_val: UnsafeCell { .. },
                                        },
                                    ),
                                },
                                map: [],
                                patset: Pool(
                                    Pool {
                                        stacks: [
                                            CacheLine(
                                                Mutex {
                                                    data: [],
                                                    poisoned: false,
                                                    ..
                                                },
                                            ),
                                            CacheLine(
                                                Mutex {
                                                    data: [],
                                                    poisoned: false,
                                                    ..
                                                },
                                            ),
                                            CacheLine(
                                                Mutex {
                                                    data: [],
                                                    poisoned: false,
                                                    ..
                                                },
                                            ),
                                            CacheLine(
                                                Mutex {
                                                    data: [],
                                                    poisoned: false,
                                                    ..
                                                },
                                            ),
                                            CacheLine(
                                                Mutex {
                                                    data: [],
                                                    poisoned: false,
                                                    ..
                                                },
                                            ),
                                            CacheLine(
                                                Mutex {
                                                    data: [],
                                                    poisoned: false,
                                                    ..
                                                },
                                            ),
                                            CacheLine(
                                                Mutex {
                                                    data: [],
                                                    poisoned: false,
                                                    ..
                                                },
                                            ),
                                            CacheLine(
                                                Mutex {
                                                    data: [],
                                                    poisoned: false,
                                                    ..
                                                },
                                            ),
                                        ],
                                        owner: 0,
                                        owner_val: UnsafeCell { .. },
                                    },
                                ),
                            },
                        ),
                    ],
                },
                poisoned: false,
                ..
            },
        },
    },
    run_conditions: RunConditions {
        never: false,
        always: false,
        ignore_broken_dep_steps: false,
        ignore_missing_outputs: true,
    },
    pipeline_rundir: XvcPath(
        "",
    ),
    terminate_timeout_processes: true,
    algorithm: Blake3,
    command_process: RwLock {
        data: CommandProcess {
            environment: {},
            step: XvcStep {
                name: "average-age",
            },
            step_command: XvcStepCommand {
                command: "sqlite3 people.db 'SELECT AVG(Age) FROM People;'",
            },
            birth: None,
            process: None,
            stdout_sender: Sender { .. },
            stderr_sender: Sender { .. },
            stdout_receiver: Receiver { .. },
            stderr_receiver: Receiver { .. },
        },
        poisoned: false,
        ..
    },
    available_process_slots: RwLock {
        data: 4,
        poisoned: false,
        ..
    },
    process_poll_milliseconds: 10,
    dependency_diffs: RwLock {
        data: HStore {
            map: {
                XvcEntity(
                    3,
                    9022053985516504033,
                ): RecordMissing {
                    actual: SqliteQueryDigest(
                        SqliteQueryDep {
                            path: XvcPath(
                                "people.db",
                            ),
                            query: "SELECT count(*) FROM People;",
                            query_digest: Some(
                                ContentDigest(
                                    XvcDigest {
                                        algorithm: Blake3,
                                        digest: [
                                            11,
                                            225,
                                            214,
                                            145,
                                            174,
                                            151,
                                            99,
                                            217,
                                            216,
                                            197,
                                            211,
                                            26,
                                            216,
                                            218,
                                            115,
                                            209,
                                            161,
                                            95,
                                            15,
                                            52,
                                            174,
                                            24,
                                            193,
                                            209,
                                            218,
                                            91,
                                            154,
                                            207,
                                            247,
                                            217,
                                            245,
                                            9,
                                        ],
                                    },
                                ),
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        8192,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1722024927,
                                            tv_nsec: 262703931,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                },
            },
        },
        poisoned: false,
        ..
    },
    output_diffs: RwLock {
        data: HStore {
            map: {},
        },
        poisoned: false,
        ..
    },
    step_e: XvcEntity(
        2,
        9270302598983016314,
    ),
    step: XvcStep {
        name: "average-age",
    },
    step_command: XvcStepCommand {
        command: "sqlite3 people.db 'SELECT AVG(Age) FROM People;'",
    },
    current_states: RwLock {
        data: HStore {
            map: {
                XvcEntity(
                    2,
                    9270302598983016314,
                ): WaitingToRun(
                    FromDiffsHasChanged,
                ),
            },
        },
        poisoned: false,
        ..
    },
    step_timeout: 10000s,
    all_steps: HStore {
        map: {
            XvcEntity(
                2,
                9270302598983016314,
            ): XvcStep {
                name: "average-age",
            },
        },
    },
    recorded_dependencies: R1NStore {
        parents: XvcStore {
            map: {
                XvcEntity(
                    2,
                    9270302598983016314,
                ): XvcStep {
                    name: "average-age",
                },
            },
            entity_index: {
                XvcStep {
                    name: "average-age",
                }: [
                    XvcEntity(
                        2,
                        9270302598983016314,
                    ),
                ],
            },
            previous: EventLog(
                [
                    Add {
                        entity: XvcEntity(
                            2,
                            9270302598983016314,
                        ),
                        value: XvcStep {
                            name: "average-age",
                        },
                    },
                    Add {
                        entity: XvcEntity(
                            2,
                            9270302598983016314,
                        ),
                        value: XvcStep {
                            name: "average-age",
                        },
                    },
                ],
            ),
            current: EventLog(
                [],
            ),
        },
        children: XvcStore {
            map: {
                XvcEntity(
                    3,
                    9022053985516504033,
                ): SqliteQueryDigest(
                    SqliteQueryDep {
                        path: XvcPath(
                            "people.db",
                        ),
                        query: "SELECT count(*) FROM People;",
                        query_digest: None,
                        xvc_metadata: None,
                    },
                ),
            },
            entity_index: {
                SqliteQueryDigest(
                    SqliteQueryDep {
                        path: XvcPath(
                            "people.db",
                        ),
                        query: "SELECT count(*) FROM People;",
                        query_digest: None,
                        xvc_metadata: None,
                    },
                ): [
                    XvcEntity(
                        3,
                        9022053985516504033,
                    ),
                ],
            },
            previous: EventLog(
                [
                    Add {
                        entity: XvcEntity(
                            3,
                            9022053985516504033,
                        ),
                        value: SqliteQueryDigest(
                            SqliteQueryDep {
                                path: XvcPath(
                                    "people.db",
                                ),
                                query: "SELECT count(*) FROM People;",
                                query_digest: None,
                                xvc_metadata: None,
                            },
                        ),
                    },
                ],
            ),
            current: EventLog(
                [],
            ),
        },
        child_parents: XvcStore {
            map: {
                XvcEntity(
                    3,
                    9022053985516504033,
                ): ChildEntity(
                    XvcEntity(
                        2,
                        9270302598983016314,
                    ),
                    PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                    PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                ),
            },
            entity_index: {
                ChildEntity(
                    XvcEntity(
                        2,
                        9270302598983016314,
                    ),
                    PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                    PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                ): [
                    XvcEntity(
                        3,
                        9022053985516504033,
                    ),
                ],
            },
            previous: EventLog(
                [
                    Add {
                        entity: XvcEntity(
                            3,
                            9022053985516504033,
                        ),
                        value: ChildEntity(
                            XvcEntity(
                                2,
                                9270302598983016314,
                            ),
                            PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                            PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                        ),
                    },
                ],
            ),
            current: EventLog(
                [],
            ),
        },
    },
    step_dependencies: {},
    step_outputs: HStore {
        map: {},
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::770] step.name: "average-age"
[TRACE][pipeline/src/pipeline/mod.rs::771] &r_next_state: Running(
    FromStartProcess,
)
[TRACE][pipeline/src/pipeline/mod.rs::773] &step_state: Running(
    FromStartProcess,
)
[TRACE][pipeline/src/pipeline/mod.rs::660] &step_state: Running(
    FromStartProcess,
)
[TRACE][pipeline/src/pipeline/command.rs::96] self.environment: {}
[TRACE][pipeline/src/pipeline/mod.rs::770] step.name: "average-age"
[TRACE][pipeline/src/pipeline/mod.rs::771] &r_next_state: Running(
    FromWaitProcess,
)
[TRACE][pipeline/src/pipeline/mod.rs::773] &step_state: Running(
    FromWaitProcess,
)
[TRACE][pipeline/src/pipeline/mod.rs::660] &step_state: Running(
    FromWaitProcess,
)
[TRACE][pipeline/src/pipeline/mod.rs::1418] params: StepStateParams {
    xvc_root: XvcRootInner {
        absolute_path: AbsolutePath(
            "[CWD]",
        ),
        xvc_dir: AbsolutePath(
            "[CWD]/.xvc",
        ),
        store_dir: AbsolutePath(
            "[CWD]/.xvc/store",
        ),
        config: XvcConfig {
            current_dir: XvcConfigOption {
                source: Runtime,
                option: AbsolutePath(
                    "[CWD]",
                ),
            },
            config_maps: [
                XvcConfigMap {
                    source: Default,
                    map: {
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "core.verbosity": String(
                            "error",
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "file.list.sort": String(
                            "name-desc",
                        ),
                        "git.use_git": Boolean(
                            true,
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "core.guid": String(
                            "919ce8649f3e32f7",
                        ),
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "pipeline.process_pool_size": Integer(
                            4,
                        ),
                        "git.command": String(
                            "git",
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "file.list.show_dot_files": Boolean(
                            false,
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                        "file.list.no_summary": Boolean(
                            false,
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                    },
                },
                XvcConfigMap {
                    source: Project,
                    map: {
                        "core.guid": String(
                            "59a2d95849757118",
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "git.command": String(
                            "git",
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "git.use_git": Boolean(
                            true,
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "file.list.no_summary": Boolean(
                            false,
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "file.list.show_dot_files": Boolean(
                            false,
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "file.list.sort": String(
                            "name-desc",
                        ),
                        "pipeline.process_pool_size": Integer(
                            4,
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "core.verbosity": String(
                            "error",
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                    },
                },
                XvcConfigMap {
                    source: Local,
                    map: {},
                },
                XvcConfigMap {
                    source: Environment,
                    map: {
                        "TRYCMD_TESTS": String(
                            "storage,file,pipeline,core",
                        ),
                    },
                },
                XvcConfigMap {
                    source: CommandLine,
                    map: {
                        "core.quiet": Boolean(
                            false,
                        ),
                        "core.verbosity": String(
                            "debug",
                        ),
                    },
                },
            ],
            the_config: {
                "pipeline.default_params_file": XvcConfigValue {
                    source: Project,
                    value: String(
                        "params.yaml",
                    ),
                },
                "file.list.sort": XvcConfigValue {
                    source: Project,
                    value: String(
                        "name-desc",
                    ),
                },
                "git.use_git": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        true,
                    ),
                },
                "git.auto_commit": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        true,
                    ),
                },
                "core.verbosity": XvcConfigValue {
                    source: CommandLine,
                    value: String(
                        "debug",
                    ),
                },
                "git.auto_stage": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "file.carry-in.force": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "pipeline.current_pipeline": XvcConfigValue {
                    source: Project,
                    value: String(
                        "default",
                    ),
                },
                "cache.algorithm": XvcConfigValue {
                    source: Project,
                    value: String(
                        "blake3",
                    ),
                },
                "pipeline.process_pool_size": XvcConfigValue {
                    source: Project,
                    value: Integer(
                        4,
                    ),
                },
                "file.list.no_summary": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "file.track.no_parallel": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "file.recheck.method": XvcConfigValue {
                    source: Project,
                    value: String(
                        "copy",
                    ),
                },
                "core.quiet": XvcConfigValue {
                    source: CommandLine,
                    value: Boolean(
                        false,
                    ),
                },
                "file.carry-in.no_parallel": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "core.guid": XvcConfigValue {
                    source: Project,
                    value: String(
                        "59a2d95849757118",
                    ),
                },
                "pipeline.default": XvcConfigValue {
                    source: Project,
                    value: String(
                        "default",
                    ),
                },
                "git.command": XvcConfigValue {
                    source: Project,
                    value: String(
                        "git",
                    ),
                },
                "file.track.force": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "file.track.no_commit": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "file.list.recursive": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "file.list.format": XvcConfigValue {
                    source: Project,
                    value: String(
                        "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                    ),
                },
                "file.track.text_or_binary": XvcConfigValue {
                    source: Project,
                    value: String(
                        "auto",
                    ),
                },
                "TRYCMD_TESTS": XvcConfigValue {
                    source: Environment,
                    value: String(
                        "storage,file,pipeline,core",
                    ),
                },
                "file.list.show_dot_files": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
            },
            init_params: XvcConfigParams {
                default_configuration: "
[core]
# The repository id. Please do not delete or change it.
# This is used to identify the repository and generate paths in storages.
# In the future it may be used to in other ways.
guid = /"919ce8649f3e32f7/"
# Default verbosity level.
# One of /"error/", /"warn/", /"info/"
verbosity = /"error/"

[git]
# Automate git operations.
# Turning this off leads Xvc to behave as if it's not in a Git repository.
# Not recommended unless you're really not using Git
use_git = true
# Command to run Git process.
# You can set this to an absolute path to specify an executable
# If set to a non-absolute path, the executable will be searched in $PATH.
command = /"git/"

# Commit changes in .xvc/ directory after commands.
# You can set this to false if you want to commit manually.
auto_commit = true

# Stage changes in .xvc/ directory without committing.
# auto_commit implies auto_stage.
# If you want to commit manually but don't want to stage after individual Xvc commands, you can set this to true.
auto_stage = false

[cache]
# The hash algorithm used for the cache.
# It may take blake3, blake2, sha2 or sha3 as values.
# All algorithms are selected to produce 256-bit hashes, so sha2 means SHA2-256, blake2 means BLAKE2s, etc.
# The cache path is produced by prepending algorithm name to the cache.
# Blake3 files are in .xvc/b3/, while sha2 files are in .xvc/s2/ etc.
algorithm = /"blake3/"

[file]

[file.track]

# Don't move file content to cache after xvc file track
no_commit = false
# Force to track files even if they are already tracked.
force = false

# Xvc calculates file content digest differently for text and binary files.
# This option controls whether to treat files as text or binary.
# It may take auto, text or binary as values.
# Auto check each file individually and treat it as text if it's text.
text_or_binary = /"auto/"

# Don't use parallelism in track operations.
# Note that some of the operations are implemented in parallel by default, and this option affects some heavier operations.
no_parallel = false

[file.list]

# Format for `xvc file list` rows. You can reorder or remove columns.
# The following are the keys for each row:
# - {acd64}:  actual content digest. All 64 digits from the workspace file's content.
# - {acd8}:  actual content digest. First 8 digits the file content digest.
# - {aft}:  actual file type. Whether the entry is a file (F), directory (D),
#   symlink (S), hardlink (H) or reflink (R).
# - {asz}:  actual size. The size of the workspace file in bytes. It uses MB,
#   GB and TB to represent sizes larger than 1MB.
# - {ats}:  actual timestamp. The timestamp of the workspace file.
# - {cst}:  cache status. One of /"=/", /">/", /"</", /"X/", or /"?/" to show
#   whether the file timestamp is the same as the cached timestamp, newer,
#   older, not cached or not tracked.
# - {name}: The name of the file or directory.
# - {rcd64}:  recorded content digest. All 64 digits.
# - {rcd8}:  recorded content digest. First 8 digits.
# - {rrm}:  recorded recheck method. Whether the entry is linked to the workspace
#   as a copy (C), symlink (S), hardlink (H) or reflink (R).
# - {rsz}:  recorded size. The size of the cached content in bytes. It uses
#   MB, GB and TB to represent sizes larged than 1MB.
# - {rts}:  recorded timestamp. The timestamp of the cached content.
#
# There are no escape sequences in the format string.
# If you want to add a tab, type it to the string.
# If you want to add a literal double curly brace, open an issue.
format = /"{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}/"

# Default sort order for `xvc file list`.
# Valid values are
# none, name-asc, name-desc, size-asc, size-desc, ts-asc, ts-desc.
sort = /"name-desc/"

# Show dot files like .gitignore
show_dot_files = false

# Do not show a summary for as the final row for `xvc file list`.
no_summary = false

# List files recursively always.
recursive = false

[file.carry-in]
# Carry-in the files to cache always, even if they are already present.
force = false

# Don't use parallel move/copy in carry-in
no_parallel = false

[file.recheck]
# The recheck method for Xvc. It may take copy, hardlink, symlink, reflink as values.
# The default is copy to make sure the options is portable.
# Copy duplicates the file content, while hardlink, symlink and reflink only create a new path to the file.
# Note that hardlink and symlink are read-only as they link the files in cache.
method = /"copy/"

[pipeline]
# Name of the current pipeline to run
current_pipeline = /"default/"
# Name of the default pipeline
default = /"default/"
# Name of the default params file name
default_params_file = /"params.yaml/"
# Number of command processes to run concurrently
process_pool_size = 4
#

",
                current_dir: AbsolutePath(
                    "[CWD]",
                ),
                include_system_config: true,
                include_user_config: true,
                project_config_path: Some(
                    AbsolutePath(
                        "[CWD]/.xvc/config.toml",
                    ),
                ),
                local_config_path: Some(
                    AbsolutePath(
                        "[CWD]/.xvc/config.local.toml",
                    ),
                ),
                include_environment_config: true,
                command_line_config: Some(
                    [
                        "core.verbosity = debug",
                        "core.quiet = false",
                    ],
                ),
            },
        },
        local_config_path: AbsolutePath(
            "[CWD]/.xvc/config.local.toml",
        ),
        project_config_path: AbsolutePath(
            "[CWD]/.xvc/config.toml",
        ),
        entity_generator: XvcEntityGenerator {
            counter: 4,
            random: 13513277701724141613,
            dirty: false,
        },
    },
    output_snd: Sender { .. },
    pmp: XvcPathMetadataProvider {
        xvc_root: XvcRootInner {
            absolute_path: AbsolutePath(
                "[CWD]",
            ),
            xvc_dir: AbsolutePath(
                "[CWD]/.xvc",
            ),
            store_dir: AbsolutePath(
                "[CWD]/.xvc/store",
            ),
            config: XvcConfig {
                current_dir: XvcConfigOption {
                    source: Runtime,
                    option: AbsolutePath(
                        "[CWD]",
                    ),
                },
                config_maps: [
                    XvcConfigMap {
                        source: Default,
                        map: {
                            "pipeline.default_params_file": String(
                                "params.yaml",
                            ),
                            "file.carry-in.no_parallel": Boolean(
                                false,
                            ),
                            "file.carry-in.force": Boolean(
                                false,
                            ),
                            "core.verbosity": String(
                                "error",
                            ),
                            "file.recheck.method": String(
                                "copy",
                            ),
                            "file.list.sort": String(
                                "name-desc",
                            ),
                            "git.use_git": Boolean(
                                true,
                            ),
                            "pipeline.current_pipeline": String(
                                "default",
                            ),
                            "core.guid": String(
                                "919ce8649f3e32f7",
                            ),
                            "git.auto_commit": Boolean(
                                true,
                            ),
                            "pipeline.default": String(
                                "default",
                            ),
                            "pipeline.process_pool_size": Integer(
                                4,
                            ),
                            "git.command": String(
                                "git",
                            ),
                            "file.track.no_commit": Boolean(
                                false,
                            ),
                            "file.track.force": Boolean(
                                false,
                            ),
                            "file.list.show_dot_files": Boolean(
                                false,
                            ),
                            "file.list.recursive": Boolean(
                                false,
                            ),
                            "file.list.format": String(
                                "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                            ),
                            "cache.algorithm": String(
                                "blake3",
                            ),
                            "git.auto_stage": Boolean(
                                false,
                            ),
                            "file.list.no_summary": Boolean(
                                false,
                            ),
                            "file.track.text_or_binary": String(
                                "auto",
                            ),
                            "file.track.no_parallel": Boolean(
                                false,
                            ),
                        },
                    },
                    XvcConfigMap {
                        source: Project,
                        map: {
                            "core.guid": String(
                                "59a2d95849757118",
                            ),
                            "cache.algorithm": String(
                                "blake3",
                            ),
                            "file.list.recursive": Boolean(
                                false,
                            ),
                            "git.command": String(
                                "git",
                            ),
                            "pipeline.default_params_file": String(
                                "params.yaml",
                            ),
                            "file.track.force": Boolean(
                                false,
                            ),
                            "file.track.no_parallel": Boolean(
                                false,
                            ),
                            "git.use_git": Boolean(
                                true,
                            ),
                            "file.track.no_commit": Boolean(
                                false,
                            ),
                            "file.recheck.method": String(
                                "copy",
                            ),
                            "file.list.no_summary": Boolean(
                                false,
                            ),
                            "pipeline.current_pipeline": String(
                                "default",
                            ),
                            "git.auto_stage": Boolean(
                                false,
                            ),
                            "git.auto_commit": Boolean(
                                true,
                            ),
                            "file.list.show_dot_files": Boolean(
                                false,
                            ),
                            "file.list.format": String(
                                "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                            ),
                            "file.list.sort": String(
                                "name-desc",
                            ),
                            "pipeline.process_pool_size": Integer(
                                4,
                            ),
                            "file.carry-in.no_parallel": Boolean(
                                false,
                            ),
                            "pipeline.default": String(
                                "default",
                            ),
                            "file.carry-in.force": Boolean(
                                false,
                            ),
                            "core.verbosity": String(
                                "error",
                            ),
                            "file.track.text_or_binary": String(
                                "auto",
                            ),
                        },
                    },
                    XvcConfigMap {
                        source: Local,
                        map: {},
                    },
                    XvcConfigMap {
                        source: Environment,
                        map: {
                            "TRYCMD_TESTS": String(
                                "storage,file,pipeline,core",
                            ),
                        },
                    },
                    XvcConfigMap {
                        source: CommandLine,
                        map: {
                            "core.quiet": Boolean(
                                false,
                            ),
                            "core.verbosity": String(
                                "debug",
                            ),
                        },
                    },
                ],
                the_config: {
                    "pipeline.default_params_file": XvcConfigValue {
                        source: Project,
                        value: String(
                            "params.yaml",
                        ),
                    },
                    "file.list.sort": XvcConfigValue {
                        source: Project,
                        value: String(
                            "name-desc",
                        ),
                    },
                    "git.use_git": XvcConfigValue {
                        source: Project,
                        value: Boolean(
                            true,
                        ),
                    },
                    "git.auto_commit": XvcConfigValue {
                        source: Project,
                        value: Boolean(
                            true,
                        ),
                    },
                    "core.verbosity": XvcConfigValue {
                        source: CommandLine,
                        value: String(
                            "debug",
                        ),
                    },
                    "git.auto_stage": XvcConfigValue {
                        source: Project,
                        value: Boolean(
                            false,
                        ),
                    },
                    "file.carry-in.force": XvcConfigValue {
                        source: Project,
                        value: Boolean(
                            false,
                        ),
                    },
                    "pipeline.current_pipeline": XvcConfigValue {
                        source: Project,
                        value: String(
                            "default",
                        ),
                    },
                    "cache.algorithm": XvcConfigValue {
                        source: Project,
                        value: String(
                            "blake3",
                        ),
                    },
                    "pipeline.process_pool_size": XvcConfigValue {
                        source: Project,
                        value: Integer(
                            4,
                        ),
                    },
                    "file.list.no_summary": XvcConfigValue {
                        source: Project,
                        value: Boolean(
                            false,
                        ),
                    },
                    "file.track.no_parallel": XvcConfigValue {
                        source: Project,
                        value: Boolean(
                            false,
                        ),
                    },
                    "file.recheck.method": XvcConfigValue {
                        source: Project,
                        value: String(
                            "copy",
                        ),
                    },
                    "core.quiet": XvcConfigValue {
                        source: CommandLine,
                        value: Boolean(
                            false,
                        ),
                    },
                    "file.carry-in.no_parallel": XvcConfigValue {
                        source: Project,
                        value: Boolean(
                            false,
                        ),
                    },
                    "core.guid": XvcConfigValue {
                        source: Project,
                        value: String(
                            "59a2d95849757118",
                        ),
                    },
                    "pipeline.default": XvcConfigValue {
                        source: Project,
                        value: String(
                            "default",
                        ),
                    },
                    "git.command": XvcConfigValue {
                        source: Project,
                        value: String(
                            "git",
                        ),
                    },
                    "file.track.force": XvcConfigValue {
                        source: Project,
                        value: Boolean(
                            false,
                        ),
                    },
                    "file.track.no_commit": XvcConfigValue {
                        source: Project,
                        value: Boolean(
                            false,
                        ),
                    },
                    "file.list.recursive": XvcConfigValue {
                        source: Project,
                        value: Boolean(
                            false,
                        ),
                    },
                    "file.list.format": XvcConfigValue {
                        source: Project,
                        value: String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                    },
                    "file.track.text_or_binary": XvcConfigValue {
                        source: Project,
                        value: String(
                            "auto",
                        ),
                    },
                    "TRYCMD_TESTS": XvcConfigValue {
                        source: Environment,
                        value: String(
                            "storage,file,pipeline,core",
                        ),
                    },
                    "file.list.show_dot_files": XvcConfigValue {
                        source: Project,
                        value: Boolean(
                            false,
                        ),
                    },
                },
                init_params: XvcConfigParams {
                    default_configuration: "
[core]
# The repository id. Please do not delete or change it.
# This is used to identify the repository and generate paths in storages.
# In the future it may be used to in other ways.
guid = /"919ce8649f3e32f7/"
# Default verbosity level.
# One of /"error/", /"warn/", /"info/"
verbosity = /"error/"

[git]
# Automate git operations.
# Turning this off leads Xvc to behave as if it's not in a Git repository.
# Not recommended unless you're really not using Git
use_git = true
# Command to run Git process.
# You can set this to an absolute path to specify an executable
# If set to a non-absolute path, the executable will be searched in $PATH.
command = /"git/"

# Commit changes in .xvc/ directory after commands.
# You can set this to false if you want to commit manually.
auto_commit = true

# Stage changes in .xvc/ directory without committing.
# auto_commit implies auto_stage.
# If you want to commit manually but don't want to stage after individual Xvc commands, you can set this to true.
auto_stage = false

[cache]
# The hash algorithm used for the cache.
# It may take blake3, blake2, sha2 or sha3 as values.
# All algorithms are selected to produce 256-bit hashes, so sha2 means SHA2-256, blake2 means BLAKE2s, etc.
# The cache path is produced by prepending algorithm name to the cache.
# Blake3 files are in .xvc/b3/, while sha2 files are in .xvc/s2/ etc.
algorithm = /"blake3/"

[file]

[file.track]

# Don't move file content to cache after xvc file track
no_commit = false
# Force to track files even if they are already tracked.
force = false

# Xvc calculates file content digest differently for text and binary files.
# This option controls whether to treat files as text or binary.
# It may take auto, text or binary as values.
# Auto check each file individually and treat it as text if it's text.
text_or_binary = /"auto/"

# Don't use parallelism in track operations.
# Note that some of the operations are implemented in parallel by default, and this option affects some heavier operations.
no_parallel = false

[file.list]

# Format for `xvc file list` rows. You can reorder or remove columns.
# The following are the keys for each row:
# - {acd64}:  actual content digest. All 64 digits from the workspace file's content.
# - {acd8}:  actual content digest. First 8 digits the file content digest.
# - {aft}:  actual file type. Whether the entry is a file (F), directory (D),
#   symlink (S), hardlink (H) or reflink (R).
# - {asz}:  actual size. The size of the workspace file in bytes. It uses MB,
#   GB and TB to represent sizes larger than 1MB.
# - {ats}:  actual timestamp. The timestamp of the workspace file.
# - {cst}:  cache status. One of /"=/", /">/", /"</", /"X/", or /"?/" to show
#   whether the file timestamp is the same as the cached timestamp, newer,
#   older, not cached or not tracked.
# - {name}: The name of the file or directory.
# - {rcd64}:  recorded content digest. All 64 digits.
# - {rcd8}:  recorded content digest. First 8 digits.
# - {rrm}:  recorded recheck method. Whether the entry is linked to the workspace
#   as a copy (C), symlink (S), hardlink (H) or reflink (R).
# - {rsz}:  recorded size. The size of the cached content in bytes. It uses
#   MB, GB and TB to represent sizes larged than 1MB.
# - {rts}:  recorded timestamp. The timestamp of the cached content.
#
# There are no escape sequences in the format string.
# If you want to add a tab, type it to the string.
# If you want to add a literal double curly brace, open an issue.
format = /"{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}/"

# Default sort order for `xvc file list`.
# Valid values are
# none, name-asc, name-desc, size-asc, size-desc, ts-asc, ts-desc.
sort = /"name-desc/"

# Show dot files like .gitignore
show_dot_files = false

# Do not show a summary for as the final row for `xvc file list`.
no_summary = false

# List files recursively always.
recursive = false

[file.carry-in]
# Carry-in the files to cache always, even if they are already present.
force = false

# Don't use parallel move/copy in carry-in
no_parallel = false

[file.recheck]
# The recheck method for Xvc. It may take copy, hardlink, symlink, reflink as values.
# The default is copy to make sure the options is portable.
# Copy duplicates the file content, while hardlink, symlink and reflink only create a new path to the file.
# Note that hardlink and symlink are read-only as they link the files in cache.
method = /"copy/"

[pipeline]
# Name of the current pipeline to run
current_pipeline = /"default/"
# Name of the default pipeline
default = /"default/"
# Name of the default params file name
default_params_file = /"params.yaml/"
# Number of command processes to run concurrently
process_pool_size = 4
#

",
                    current_dir: AbsolutePath(
                        "[CWD]",
                    ),
                    include_system_config: true,
                    include_user_config: true,
                    project_config_path: Some(
                        AbsolutePath(
                            "[CWD]/.xvc/config.toml",
                        ),
                    ),
                    local_config_path: Some(
                        AbsolutePath(
                            "[CWD]/.xvc/config.local.toml",
                        ),
                    ),
                    include_environment_config: true,
                    command_line_config: Some(
                        [
                            "core.verbosity = debug",
                            "core.quiet = false",
                        ],
                    ),
                },
            },
            local_config_path: AbsolutePath(
                "[CWD]/.xvc/config.local.toml",
            ),
            project_config_path: AbsolutePath(
                "[CWD]/.xvc/config.toml",
            ),
            entity_generator: XvcEntityGenerator {
                counter: 4,
                random: 13513277701724141613,
                dirty: false,
            },
        },
        path_map: RwLock {
            data: {
                XvcPath(
                    "people.db",
                ): XvcMetadata {
                    file_type: File,
                    size: Some(
                        8192,
                    ),
                    modified: Some(
                        SystemTime {
                            tv_sec: 1722024927,
                            tv_nsec: 262703931,
                        },
                    ),
                },
            },
            poisoned: false,
            ..
        },
        kill_signal_sender: Sender { .. },
        background_thread: Mutex {
            data: JoinHandle { .. },
            poisoned: false,
            ..
        },
        output_sender: Sender { .. },
        ignore_rules: IgnoreRules {
            root: "[CWD]",
            ignore_patterns: RwLock {
                data: [
                    Pattern {
                        pattern: Glob {
                            glob: "**/.xvc",
                            re: "(?-u)^(?:/?|.*/)//.xvc$",
                            opts: GlobOptions {
                                case_insensitive: false,
                                literal_separator: false,
                                backslash_escape: true,
                                empty_alternates: false,
                            },
                            tokens: Tokens(
                                [
                                    RecursivePrefix,
                                    Literal(
                                        '.',
                                    ),
                                    Literal(
                                        'x',
                                    ),
                                    Literal(
                                        'v',
                                    ),
                                    Literal(
                                        'c',
                                    ),
                                ],
                            ),
                        },
                        original: ".xvc",
                        source: Global,
                        effect: Ignore,
                        relativity: Anywhere,
                        path_kind: Any,
                    },
                    Pattern {
                        pattern: Glob {
                            glob: "**/.git",
                            re: "(?-u)^(?:/?|.*/)//.git$",
                            opts: GlobOptions {
                                case_insensitive: false,
                                literal_separator: false,
                                backslash_escape: true,
                                empty_alternates: false,
                            },
                            tokens: Tokens(
                                [
                                    RecursivePrefix,
                                    Literal(
                                        '.',
                                    ),
                                    Literal(
                                        'g',
                                    ),
                                    Literal(
                                        'i',
                                    ),
                                    Literal(
                                        't',
                                    ),
                                ],
                            ),
                        },
                        original: ".git",
                        source: Global,
                        effect: Ignore,
                        relativity: Anywhere,
                        path_kind: Any,
                    },
                    Pattern {
                        pattern: Glob {
                            glob: "**/.DS_Store",
                            re: "(?-u)^(?:/?|.*/)//.DS_Store$",
                            opts: GlobOptions {
                                case_insensitive: false,
                                literal_separator: false,
                                backslash_escape: true,
                                empty_alternates: false,
                            },
                            tokens: Tokens(
                                [
                                    RecursivePrefix,
                                    Literal(
                                        '.',
                                    ),
                                    Literal(
                                        'D',
                                    ),
                                    Literal(
                                        'S',
                                    ),
                                    Literal(
                                        '_',
                                    ),
                                    Literal(
                                        'S',
                                    ),
                                    Literal(
                                        't',
                                    ),
                                    Literal(
                                        'o',
                                    ),
                                    Literal(
                                        'r',
                                    ),
                                    Literal(
                                        'e',
                                    ),
                                ],
                            ),
                        },
                        original: ".DS_Store",
                        source: File {
                            path: ".xvcignore",
                            line: 6,
                        },
                        effect: Ignore,
                        relativity: Anywhere,
                        path_kind: Any,
                    },
                ],
                poisoned: false,
                ..
            },
            whitelist_patterns: RwLock {
                data: [],
                poisoned: false,
                ..
            },
            whitelist_set: RwLock {
                data: GlobSet {
                    len: 0,
                    strats: [],
                },
                poisoned: false,
                ..
            },
            ignore_set: RwLock {
                data: GlobSet {
                    len: 3,
                    strats: [
                        Extension(
                            ExtensionStrategy(
                                {},
                            ),
                        ),
                        BasenameLiteral(
                            BasenameLiteralStrategy(
                                {
                                    [
                                        46,
                                        120,
                                        118,
                                        99,
                                    ]: [
                                        0,
                                    ],
                                    [
                                        46,
                                        68,
                                        83,
                                        95,
                                        83,
                                        116,
                                        111,
                                        114,
                                        101,
                                    ]: [
                                        2,
                                    ],
                                    [
                                        46,
                                        103,
                                        105,
                                        116,
                                    ]: [
                                        1,
                                    ],
                                },
                            ),
                        ),
                        Literal(
                            LiteralStrategy(
                                {},
                            ),
                        ),
                        Suffix(
                            SuffixStrategy {
                                matcher: AhoCorasick(
                                    dfa::DFA(
                                    D 000000: /x00 => 0
                                    F 000001:
                                     >000002: /x00 => 2
                                      000003: /x00 => 0
                                    match kind: Standard
                                    prefilter: false
                                    state length: 4
                                    pattern length: 0
                                    shortest pattern length: 18446744073709551615
                                    longest pattern length: 0
                                    alphabet length: 1
                                    stride: 1
                                    byte classes: ByteClasses(0 => [0-255])
                                    memory usage: 16
                                    )
                                    ,
                                ),
                                map: [],
                                longest: 0,
                            },
                        ),
                        Prefix(
                            PrefixStrategy {
                                matcher: AhoCorasick(
                                    dfa::DFA(
                                    D 000000: /x00 => 0
                                    F 000001:
                                     >000002: /x00 => 2
                                      000003: /x00 => 0
                                    match kind: Standard
                                    prefilter: false
                                    state length: 4
                                    pattern length: 0
                                    shortest pattern length: 18446744073709551615
                                    longest pattern length: 0
                                    alphabet length: 1
                                    stride: 1
                                    byte classes: ByteClasses(0 => [0-255])
                                    memory usage: 16
                                    )
                                    ,
                                ),
                                map: [],
                                longest: 0,
                            },
                        ),
                        RequiredExtension(
                            RequiredExtensionStrategy(
                                {},
                            ),
                        ),
                        Regex(
                            RegexSetStrategy {
                                matcher: Regex {
                                    imp: RegexI {
                                        strat: Core {
                                            info: RegexInfo(
                                                RegexInfoI {
                                                    config: Config {
                                                        match_kind: Some(
                                                            All,
                                                        ),
                                                        utf8_empty: Some(
                                                            false,
                                                        ),
                                                        autopre: None,
                                                        pre: None,
                                                        which_captures: None,
                                                        nfa_size_limit: Some(
                                                            Some(
                                                                10485760,
                                                            ),
                                                        ),
                                                        onepass_size_limit: None,
                                                        hybrid_cache_capacity: Some(
                                                            10485760,
                                                        ),
                                                        hybrid: None,
                                                        dfa: None,
                                                        dfa_size_limit: None,
                                                        dfa_state_limit: None,
                                                        onepass: None,
                                                        backtrack: None,
                                                        byte_classes: None,
                                                        line_terminator: None,
                                                    },
                                                    props: [],
                                                    props_union: Properties(
                                                        PropertiesI {
                                                            minimum_len: None,
                                                            maximum_len: None,
                                                            look_set: ∅,
                                                            look_set_prefix: ∅,
                                                            look_set_suffix: ∅,
                                                            look_set_prefix_any: ∅,
                                                            look_set_suffix_any: ∅,
                                                            utf8: true,
                                                            explicit_captures_len: 0,
                                                            static_explicit_captures_len: None,
                                                            literal: false,
                                                            alternation_literal: true,
                                                        },
                                                    ),
                                                },
                                            ),
                                            pre: None,
                                            nfa: thompson::NFA(
                                            ^000000: FAIL

                                            transition equivalence classes: ByteClasses(0 => [/x00-/xFF], 1 => [EOI])
                                            )
                                            ,
                                            nfarev: Some(
                                                thompson::NFA(
                                                ^000000: FAIL

                                                transition equivalence classes: ByteClasses(0 => [/x00-/xFF], 1 => [EOI])
                                                )
                                                ,
                                            ),
                                            pikevm: PikeVM(
                                                PikeVMEngine(
                                                    PikeVM {
                                                        config: Config {
                                                            match_kind: Some(
                                                                All,
                                                            ),
                                                            pre: Some(
                                                                None,
                                                            ),
                                                        },
                                                        nfa: thompson::NFA(
                                                        ^000000: FAIL

                                                        transition equivalence classes: ByteClasses(0 => [/x00-/xFF], 1 => [EOI])
                                                        )
                                                        ,
                                                    },
                                                ),
                                            ),
                                            backtrack: BoundedBacktracker(
                                                None,
                                            ),
                                            onepass: OnePass(
                                                None,
                                            ),
                                            hybrid: Hybrid(
                                                Some(
                                                    HybridEngine(
                                                        Regex {
                                                            forward: DFA {
                                                                config: Config {
                                                                    match_kind: Some(
                                                                        All,
                                                                    ),
                                                                    pre: Some(
                                                                        None,
                                                                    ),
                                                                    starts_for_each_pattern: Some(
                                                                        true,
                                                                    ),
                                                                    byte_classes: Some(
                                                                        true,
                                                                    ),
                                                                    unicode_word_boundary: Some(
                                                                        true,
                                                                    ),
                                                                    quitset: None,
                                                                    specialize_start_states: Some(
                                                                        false,
                                                                    ),
                                                                    cache_capacity: Some(
                                                                        10485760,
                                                                    ),
                                                                    skip_cache_capacity_check: Some(
                                                                        false,
                                                                    ),
                                                                    minimum_cache_clear_count: Some(
                                                                        Some(
                                                                            3,
                                                                        ),
                                                                    ),
                                                                    minimum_bytes_per_state: Some(
                                                                        Some(
                                                                            10,
                                                                        ),
                                                                    ),
                                                                },
                                                                nfa: thompson::NFA(
                                                                ^000000: FAIL

                                                                transition equivalence classes: ByteClasses(0 => [/x00-/xFF], 1 => [EOI])
                                                                )
                                                                ,
                                                                stride2: 1,
                                                                start_map: StartByteMap{/x00 => NonWordByte, /x01 => NonWordByte, /x02 => NonWordByte, /x03 => NonWordByte, /x04 => NonWordByte, /x05 => NonWordByte, /x06 => NonWordByte, /x07 => NonWordByte, /x08 => NonWordByte, /t => NonWordByte,
 => LineLF, /x0B => NonWordByte, /x0C => NonWordByte, /r => LineCR, /x0E => NonWordByte, /x0F => NonWordByte, /x10 => NonWordByte, /x11 => NonWordByte, /x12 => NonWordByte, /x13 => NonWordByte, /x14 => NonWordByte, /x15 => NonWordByte, /x16 => NonWordByte, /x17 => NonWordByte, /x18 => NonWordByte, /x19 => NonWordByte, /x1A => NonWordByte, /x1B => NonWordByte, /x1C => NonWordByte, /x1D => NonWordByte, /x1E => NonWordByte, /x1F => NonWordByte, ' ' => NonWordByte, ! => NonWordByte, /" => NonWordByte, # => NonWordByte, $ => NonWordByte, % => NonWordByte, & => NonWordByte, /' => NonWordByte, ( => NonWordByte, ) => NonWordByte, * => NonWordByte, + => NonWordByte, , => NonWordByte, - => NonWordByte, . => NonWordByte, / => NonWordByte, 0 => WordByte, 1 => WordByte, 2 => WordByte, 3 => WordByte, 4 => WordByte, 5 => WordByte, 6 => WordByte, 7 => WordByte, 8 => WordByte, 9 => WordByte, : => NonWordByte, ; => NonWordByte, < => NonWordByte, = => NonWordByte, > => NonWordByte, ? => NonWordByte, @ => NonWordByte, A => WordByte, B => WordByte, C => WordByte, D => WordByte, E => WordByte, F => WordByte, G => WordByte, H => WordByte, I => WordByte, J => WordByte, K => WordByte, L => WordByte, M => WordByte, N => WordByte, O => WordByte, P => WordByte, Q => WordByte, R => WordByte, S => WordByte, T => WordByte, U => WordByte, V => WordByte, W => WordByte, X => WordByte, Y => WordByte, Z => WordByte, [ => NonWordByte, // => NonWordByte, ] => NonWordByte, ^ => NonWordByte, _ => WordByte, ` => NonWordByte, a => WordByte, b => WordByte, c => WordByte, d => WordByte, e => WordByte, f => WordByte, g => WordByte, h => WordByte, i => WordByte, j => WordByte, k => WordByte, l => WordByte, m => WordByte, n => WordByte, o => WordByte, p => WordByte, q => WordByte, r => WordByte, s => WordByte, t => WordByte, u => WordByte, v => WordByte, w => WordByte, x => WordByte, y => WordByte, z => WordByte, { => NonWordByte, | => NonWordByte, } => NonWordByte, ~ => NonWordByte, /x7F => NonWordByte, /x80 => NonWordByte, /x81 => NonWordByte, /x82 => NonWordByte, /x83 => NonWordByte, /x84 => NonWordByte, /x85 => NonWordByte, /x86 => NonWordByte, /x87 => NonWordByte, /x88 => NonWordByte, /x89 => NonWordByte, /x8A => NonWordByte, /x8B => NonWordByte, /x8C => NonWordByte, /x8D => NonWordByte, /x8E => NonWordByte, /x8F => NonWordByte, /x90 => NonWordByte, /x91 => NonWordByte, /x92 => NonWordByte, /x93 => NonWordByte, /x94 => NonWordByte, /x95 => NonWordByte, /x96 => NonWordByte, /x97 => NonWordByte, /x98 => NonWordByte, /x99 => NonWordByte, /x9A => NonWordByte, /x9B => NonWordByte, /x9C => NonWordByte, /x9D => NonWordByte, /x9E => NonWordByte, /x9F => NonWordByte, /xA0 => NonWordByte, /xA1 => NonWordByte, /xA2 => NonWordByte, /xA3 => NonWordByte, /xA4 => NonWordByte, /xA5 => NonWordByte, /xA6 => NonWordByte, /xA7 => NonWordByte, /xA8 => NonWordByte, /xA9 => NonWordByte, /xAA => NonWordByte, /xAB => NonWordByte, /xAC => NonWordByte, /xAD => NonWordByte, /xAE => NonWordByte, /xAF => NonWordByte, /xB0 => NonWordByte, /xB1 => NonWordByte, /xB2 => NonWordByte, /xB3 => NonWordByte, /xB4 => NonWordByte, /xB5 => NonWordByte, /xB6 => NonWordByte, /xB7 => NonWordByte, /xB8 => NonWordByte, /xB9 => NonWordByte, /xBA => NonWordByte, /xBB => NonWordByte, /xBC => NonWordByte, /xBD => NonWordByte, /xBE => NonWordByte, /xBF => NonWordByte, /xC0 => NonWordByte, /xC1 => NonWordByte, /xC2 => NonWordByte, /xC3 => NonWordByte, /xC4 => NonWordByte, /xC5 => NonWordByte, /xC6 => NonWordByte, /xC7 => NonWordByte, /xC8 => NonWordByte, /xC9 => NonWordByte, /xCA => NonWordByte, /xCB => NonWordByte, /xCC => NonWordByte, /xCD => NonWordByte, /xCE => NonWordByte, /xCF => NonWordByte, /xD0 => NonWordByte, /xD1 => NonWordByte, /xD2 => NonWordByte, /xD3 => NonWordByte, /xD4 => NonWordByte, /xD5 => NonWordByte, /xD6 => NonWordByte, /xD7 => NonWordByte, /xD8 => NonWordByte, /xD9 => NonWordByte, /xDA => NonWordByte, /xDB => NonWordByte, /xDC => NonWordByte, /xDD => NonWordByte, /xDE => NonWordByte, /xDF => NonWordByte, /xE0 => NonWordByte, /xE1 => NonWordByte, /xE2 => NonWordByte, /xE3 => NonWordByte, /xE4 => NonWordByte, /xE5 => NonWordByte, /xE6 => NonWordByte, /xE7 => NonWordByte, /xE8 => NonWordByte, /xE9 => NonWordByte, /xEA => NonWordByte, /xEB => NonWordByte, /xEC => NonWordByte, /xED => NonWordByte, /xEE => NonWordByte, /xEF => NonWordByte, /xF0 => NonWordByte, /xF1 => NonWordByte, /xF2 => NonWordByte, /xF3 => NonWordByte, /xF4 => NonWordByte, /xF5 => NonWordByte, /xF6 => NonWordByte, /xF7 => NonWordByte, /xF8 => NonWordByte, /xF9 => NonWordByte, /xFA => NonWordByte, /xFB => NonWordByte, /xFC => NonWordByte, /xFD => NonWordByte, /xFE => NonWordByte, /xFF => NonWordByte},
                                                                classes: ByteClasses(0 => [/x00-/xFF], 1 => [EOI]),
                                                                quitset: ByteSet {
                                                                    bits: {},
                                                                },
                                                                cache_capacity: 10485760,
                                                            },
                                                            reverse: DFA {
                                                                config: Config {
                                                                    match_kind: Some(
                                                                        All,
                                                                    ),
                                                                    pre: Some(
                                                                        None,
                                                                    ),
                                                                    starts_for_each_pattern: Some(
                                                                        true,
                                                                    ),
                                                                    byte_classes: Some(
                                                                        true,
                                                                    ),
                                                                    unicode_word_boundary: Some(
                                                                        true,
                                                                    ),
                                                                    quitset: None,
                                                                    specialize_start_states: Some(
                                                                        false,
                                                                    ),
                                                                    cache_capacity: Some(
                                                                        10485760,
                                                                    ),
                                                                    skip_cache_capacity_check: Some(
                                                                        false,
                                                                    ),
                                                                    minimum_cache_clear_count: Some(
                                                                        Some(
                                                                            3,
                                                                        ),
                                                                    ),
                                                                    minimum_bytes_per_state: Some(
                                                                        Some(
                                                                            10,
                                                                        ),
                                                                    ),
                                                                },
                                                                nfa: thompson::NFA(
                                                                ^000000: FAIL

                                                                transition equivalence classes: ByteClasses(0 => [/x00-/xFF], 1 => [EOI])
                                                                )
                                                                ,
                                                                stride2: 1,
                                                                start_map: StartByteMap{/x00 => NonWordByte, /x01 => NonWordByte, /x02 => NonWordByte, /x03 => NonWordByte, /x04 => NonWordByte, /x05 => NonWordByte, /x06 => NonWordByte, /x07 => NonWordByte, /x08 => NonWordByte, /t => NonWordByte,
 => LineLF, /x0B => NonWordByte, /x0C => NonWordByte, /r => LineCR, /x0E => NonWordByte, /x0F => NonWordByte, /x10 => NonWordByte, /x11 => NonWordByte, /x12 => NonWordByte, /x13 => NonWordByte, /x14 => NonWordByte, /x15 => NonWordByte, /x16 => NonWordByte, /x17 => NonWordByte, /x18 => NonWordByte, /x19 => NonWordByte, /x1A => NonWordByte, /x1B => NonWordByte, /x1C => NonWordByte, /x1D => NonWordByte, /x1E => NonWordByte, /x1F => NonWordByte, ' ' => NonWordByte, ! => NonWordByte, /" => NonWordByte, # => NonWordByte, $ => NonWordByte, % => NonWordByte, & => NonWordByte, /' => NonWordByte, ( => NonWordByte, ) => NonWordByte, * => NonWordByte, + => NonWordByte, , => NonWordByte, - => NonWordByte, . => NonWordByte, / => NonWordByte, 0 => WordByte, 1 => WordByte, 2 => WordByte, 3 => WordByte, 4 => WordByte, 5 => WordByte, 6 => WordByte, 7 => WordByte, 8 => WordByte, 9 => WordByte, : => NonWordByte, ; => NonWordByte, < => NonWordByte, = => NonWordByte, > => NonWordByte, ? => NonWordByte, @ => NonWordByte, A => WordByte, B => WordByte, C => WordByte, D => WordByte, E => WordByte, F => WordByte, G => WordByte, H => WordByte, I => WordByte, J => WordByte, K => WordByte, L => WordByte, M => WordByte, N => WordByte, O => WordByte, P => WordByte, Q => WordByte, R => WordByte, S => WordByte, T => WordByte, U => WordByte, V => WordByte, W => WordByte, X => WordByte, Y => WordByte, Z => WordByte, [ => NonWordByte, // => NonWordByte, ] => NonWordByte, ^ => NonWordByte, _ => WordByte, ` => NonWordByte, a => WordByte, b => WordByte, c => WordByte, d => WordByte, e => WordByte, f => WordByte, g => WordByte, h => WordByte, i => WordByte, j => WordByte, k => WordByte, l => WordByte, m => WordByte, n => WordByte, o => WordByte, p => WordByte, q => WordByte, r => WordByte, s => WordByte, t => WordByte, u => WordByte, v => WordByte, w => WordByte, x => WordByte, y => WordByte, z => WordByte, { => NonWordByte, | => NonWordByte, } => NonWordByte, ~ => NonWordByte, /x7F => NonWordByte, /x80 => NonWordByte, /x81 => NonWordByte, /x82 => NonWordByte, /x83 => NonWordByte, /x84 => NonWordByte, /x85 => NonWordByte, /x86 => NonWordByte, /x87 => NonWordByte, /x88 => NonWordByte, /x89 => NonWordByte, /x8A => NonWordByte, /x8B => NonWordByte, /x8C => NonWordByte, /x8D => NonWordByte, /x8E => NonWordByte, /x8F => NonWordByte, /x90 => NonWordByte, /x91 => NonWordByte, /x92 => NonWordByte, /x93 => NonWordByte, /x94 => NonWordByte, /x95 => NonWordByte, /x96 => NonWordByte, /x97 => NonWordByte, /x98 => NonWordByte, /x99 => NonWordByte, /x9A => NonWordByte, /x9B => NonWordByte, /x9C => NonWordByte, /x9D => NonWordByte, /x9E => NonWordByte, /x9F => NonWordByte, /xA0 => NonWordByte, /xA1 => NonWordByte, /xA2 => NonWordByte, /xA3 => NonWordByte, /xA4 => NonWordByte, /xA5 => NonWordByte, /xA6 => NonWordByte, /xA7 => NonWordByte, /xA8 => NonWordByte, /xA9 => NonWordByte, /xAA => NonWordByte, /xAB => NonWordByte, /xAC => NonWordByte, /xAD => NonWordByte, /xAE => NonWordByte, /xAF => NonWordByte, /xB0 => NonWordByte, /xB1 => NonWordByte, /xB2 => NonWordByte, /xB3 => NonWordByte, /xB4 => NonWordByte, /xB5 => NonWordByte, /xB6 => NonWordByte, /xB7 => NonWordByte, /xB8 => NonWordByte, /xB9 => NonWordByte, /xBA => NonWordByte, /xBB => NonWordByte, /xBC => NonWordByte, /xBD => NonWordByte, /xBE => NonWordByte, /xBF => NonWordByte, /xC0 => NonWordByte, /xC1 => NonWordByte, /xC2 => NonWordByte, /xC3 => NonWordByte, /xC4 => NonWordByte, /xC5 => NonWordByte, /xC6 => NonWordByte, /xC7 => NonWordByte, /xC8 => NonWordByte, /xC9 => NonWordByte, /xCA => NonWordByte, /xCB => NonWordByte, /xCC => NonWordByte, /xCD => NonWordByte, /xCE => NonWordByte, /xCF => NonWordByte, /xD0 => NonWordByte, /xD1 => NonWordByte, /xD2 => NonWordByte, /xD3 => NonWordByte, /xD4 => NonWordByte, /xD5 => NonWordByte, /xD6 => NonWordByte, /xD7 => NonWordByte, /xD8 => NonWordByte, /xD9 => NonWordByte, /xDA => NonWordByte, /xDB => NonWordByte, /xDC => NonWordByte, /xDD => NonWordByte, /xDE => NonWordByte, /xDF => NonWordByte, /xE0 => NonWordByte, /xE1 => NonWordByte, /xE2 => NonWordByte, /xE3 => NonWordByte, /xE4 => NonWordByte, /xE5 => NonWordByte, /xE6 => NonWordByte, /xE7 => NonWordByte, /xE8 => NonWordByte, /xE9 => NonWordByte, /xEA => NonWordByte, /xEB => NonWordByte, /xEC => NonWordByte, /xED => NonWordByte, /xEE => NonWordByte, /xEF => NonWordByte, /xF0 => NonWordByte, /xF1 => NonWordByte, /xF2 => NonWordByte, /xF3 => NonWordByte, /xF4 => NonWordByte, /xF5 => NonWordByte, /xF6 => NonWordByte, /xF7 => NonWordByte, /xF8 => NonWordByte, /xF9 => NonWordByte, /xFA => NonWordByte, /xFB => NonWordByte, /xFC => NonWordByte, /xFD => NonWordByte, /xFE => NonWordByte, /xFF => NonWordByte},
                                                                classes: ByteClasses(0 => [/x00-/xFF], 1 => [EOI]),
                                                                quitset: ByteSet {
                                                                    bits: {},
                                                                },
                                                                cache_capacity: 10485760,
                                                            },
                                                        },
                                                    ),
                                                ),
                                            ),
                                            dfa: DFA(
                                                None,
                                            ),
                                        },
                                        info: RegexInfo(
                                            RegexInfoI {
                                                config: Config {
                                                    match_kind: Some(
                                                        All,
                                                    ),
                                                    utf8_empty: Some(
                                                        false,
                                                    ),
                                                    autopre: None,
                                                    pre: None,
                                                    which_captures: None,
                                                    nfa_size_limit: Some(
                                                        Some(
                                                            10485760,
                                                        ),
                                                    ),
                                                    onepass_size_limit: None,
                                                    hybrid_cache_capacity: Some(
                                                        10485760,
                                                    ),
                                                    hybrid: None,
                                                    dfa: None,
                                                    dfa_size_limit: None,
                                                    dfa_state_limit: None,
                                                    onepass: None,
                                                    backtrack: None,
                                                    byte_classes: None,
                                                    line_terminator: None,
                                                },
                                                props: [],
                                                props_union: Properties(
                                                    PropertiesI {
                                                        minimum_len: None,
                                                        maximum_len: None,
                                                        look_set: ∅,
                                                        look_set_prefix: ∅,
                                                        look_set_suffix: ∅,
                                                        look_set_prefix_any: ∅,
                                                        look_set_suffix_any: ∅,
                                                        utf8: true,
                                                        explicit_captures_len: 0,
                                                        static_explicit_captures_len: None,
                                                        literal: false,
                                                        alternation_literal: true,
                                                    },
                                                ),
                                            },
                                        ),
                                    },
                                    pool: Pool(
                                        Pool {
                                            stacks: [
                                                CacheLine(
                                                    Mutex {
                                                        data: [],
                                                        poisoned: false,
                                                        ..
                                                    },
                                                ),
                                                CacheLine(
                                                    Mutex {
                                                        data: [],
                                                        poisoned: false,
                                                        ..
                                                    },
                                                ),
                                                CacheLine(
                                                    Mutex {
                                                        data: [],
                                                        poisoned: false,
                                                        ..
                                                    },
                                                ),
                                                CacheLine(
                                                    Mutex {
                                                        data: [],
                                                        poisoned: false,
                                                        ..
                                                    },
                                                ),
                                                CacheLine(
                                                    Mutex {
                                                        data: [],
                                                        poisoned: false,
                                                        ..
                                                    },
                                                ),
                                                CacheLine(
                                                    Mutex {
                                                        data: [],
                                                        poisoned: false,
                                                        ..
                                                    },
                                                ),
                                                CacheLine(
                                                    Mutex {
                                                        data: [],
                                                        poisoned: false,
                                                        ..
                                                    },
                                                ),
                                                CacheLine(
                                                    Mutex {
                                                        data: [],
                                                        poisoned: false,
                                                        ..
                                                    },
                                                ),
                                            ],
                                            owner: 0,
                                            owner_val: UnsafeCell { .. },
                                        },
                                    ),
                                },
                                map: [],
                                patset: Pool(
                                    Pool {
                                        stacks: [
                                            CacheLine(
                                                Mutex {
                                                    data: [],
                                                    poisoned: false,
                                                    ..
                                                },
                                            ),
                                            CacheLine(
                                                Mutex {
                                                    data: [],
                                                    poisoned: false,
                                                    ..
                                                },
                                            ),
                                            CacheLine(
                                                Mutex {
                                                    data: [],
                                                    poisoned: false,
                                                    ..
                                                },
                                            ),
                                            CacheLine(
                                                Mutex {
                                                    data: [],
                                                    poisoned: false,
                                                    ..
                                                },
                                            ),
                                            CacheLine(
                                                Mutex {
                                                    data: [],
                                                    poisoned: false,
                                                    ..
                                                },
                                            ),
                                            CacheLine(
                                                Mutex {
                                                    data: [],
                                                    poisoned: false,
                                                    ..
                                                },
                                            ),
                                            CacheLine(
                                                Mutex {
                                                    data: [],
                                                    poisoned: false,
                                                    ..
                                                },
                                            ),
                                            CacheLine(
                                                Mutex {
                                                    data: [],
                                                    poisoned: false,
                                                    ..
                                                },
                                            ),
                                        ],
                                        owner: 0,
                                        owner_val: UnsafeCell { .. },
                                    },
                                ),
                            },
                        ),
                    ],
                },
                poisoned: false,
                ..
            },
        },
    },
    run_conditions: RunConditions {
        never: false,
        always: false,
        ignore_broken_dep_steps: false,
        ignore_missing_outputs: true,
    },
    pipeline_rundir: XvcPath(
        "",
    ),
    terminate_timeout_processes: true,
    algorithm: Blake3,
    command_process: RwLock {
        data: CommandProcess {
            environment: {},
            step: XvcStep {
                name: "average-age",
            },
            step_command: XvcStepCommand {
                command: "sqlite3 people.db 'SELECT AVG(Age) FROM People;'",
            },
            birth: Some(
                Instant {
                    tv_sec: 834347,
                    tv_nsec: 663124791,
                },
            ),
            process: Some(
                Popen {
                    stdin: None,
                    stdout: Some(
                        File {
                            fd: 5,
                            read: true,
                            write: false,
                        },
                    ),
                    stderr: Some(
                        File {
                            fd: 7,
                            read: true,
                            write: false,
                        },
                    ),
                    child_state: Running {
                        pid: 21337,
                        ext: (),
                    },
                    detached: true,
                },
            ),
            stdout_sender: Sender { .. },
            stderr_sender: Sender { .. },
            stdout_receiver: Receiver { .. },
            stderr_receiver: Receiver { .. },
        },
        poisoned: false,
        ..
    },
    available_process_slots: RwLock {
        data: 3,
        poisoned: false,
        ..
    },
    process_poll_milliseconds: 10,
    dependency_diffs: RwLock {
        data: HStore {
            map: {
                XvcEntity(
                    3,
                    9022053985516504033,
                ): RecordMissing {
                    actual: SqliteQueryDigest(
                        SqliteQueryDep {
                            path: XvcPath(
                                "people.db",
                            ),
                            query: "SELECT count(*) FROM People;",
                            query_digest: Some(
                                ContentDigest(
                                    XvcDigest {
                                        algorithm: Blake3,
                                        digest: [
                                            11,
                                            225,
                                            214,
                                            145,
                                            174,
                                            151,
                                            99,
                                            217,
                                            216,
                                            197,
                                            211,
                                            26,
                                            216,
                                            218,
                                            115,
                                            209,
                                            161,
                                            95,
                                            15,
                                            52,
                                            174,
                                            24,
                                            193,
                                            209,
                                            218,
                                            91,
                                            154,
                                            207,
                                            247,
                                            217,
                                            245,
                                            9,
                                        ],
                                    },
                                ),
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        8192,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1722024927,
                                            tv_nsec: 262703931,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                },
            },
        },
        poisoned: false,
        ..
    },
    output_diffs: RwLock {
        data: HStore {
            map: {},
        },
        poisoned: false,
        ..
    },
    step_e: XvcEntity(
        2,
        9270302598983016314,
    ),
    step: XvcStep {
        name: "average-age",
    },
    step_command: XvcStepCommand {
        command: "sqlite3 people.db 'SELECT AVG(Age) FROM People;'",
    },
    current_states: RwLock {
        data: HStore {
            map: {
                XvcEntity(
                    2,
                    9270302598983016314,
                ): Running(
                    FromWaitProcess,
                ),
            },
        },
        poisoned: false,
        ..
    },
    step_timeout: 10000s,
    all_steps: HStore {
        map: {
            XvcEntity(
                2,
                9270302598983016314,
            ): XvcStep {
                name: "average-age",
            },
        },
    },
    recorded_dependencies: R1NStore {
        parents: XvcStore {
            map: {
                XvcEntity(
                    2,
                    9270302598983016314,
                ): XvcStep {
                    name: "average-age",
                },
            },
            entity_index: {
                XvcStep {
                    name: "average-age",
                }: [
                    XvcEntity(
                        2,
                        9270302598983016314,
                    ),
                ],
            },
            previous: EventLog(
                [
                    Add {
                        entity: XvcEntity(
                            2,
                            9270302598983016314,
                        ),
                        value: XvcStep {
                            name: "average-age",
                        },
                    },
                    Add {
                        entity: XvcEntity(
                            2,
                            9270302598983016314,
                        ),
                        value: XvcStep {
                            name: "average-age",
                        },
                    },
                ],
            ),
            current: EventLog(
                [],
            ),
        },
        children: XvcStore {
            map: {
                XvcEntity(
                    3,
                    9022053985516504033,
                ): SqliteQueryDigest(
                    SqliteQueryDep {
                        path: XvcPath(
                            "people.db",
                        ),
                        query: "SELECT count(*) FROM People;",
                        query_digest: None,
                        xvc_metadata: None,
                    },
                ),
            },
            entity_index: {
                SqliteQueryDigest(
                    SqliteQueryDep {
                        path: XvcPath(
                            "people.db",
                        ),
                        query: "SELECT count(*) FROM People;",
                        query_digest: None,
                        xvc_metadata: None,
                    },
                ): [
                    XvcEntity(
                        3,
                        9022053985516504033,
                    ),
                ],
            },
            previous: EventLog(
                [
                    Add {
                        entity: XvcEntity(
                            3,
                            9022053985516504033,
                        ),
                        value: SqliteQueryDigest(
                            SqliteQueryDep {
                                path: XvcPath(
                                    "people.db",
                                ),
                                query: "SELECT count(*) FROM People;",
                                query_digest: None,
                                xvc_metadata: None,
                            },
                        ),
                    },
                ],
            ),
            current: EventLog(
                [],
            ),
        },
        child_parents: XvcStore {
            map: {
                XvcEntity(
                    3,
                    9022053985516504033,
                ): ChildEntity(
                    XvcEntity(
                        2,
                        9270302598983016314,
                    ),
                    PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                    PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                ),
            },
            entity_index: {
                ChildEntity(
                    XvcEntity(
                        2,
                        9270302598983016314,
                    ),
                    PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                    PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                ): [
                    XvcEntity(
                        3,
                        9022053985516504033,
                    ),
                ],
            },
            previous: EventLog(
                [
                    Add {
                        entity: XvcEntity(
                            3,
                            9022053985516504033,
                        ),
                        value: ChildEntity(
                            XvcEntity(
                                2,
                                9270302598983016314,
                            ),
                            PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                            PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                        ),
                    },
                ],
            ),
            current: EventLog(
                [],
            ),
        },
    },
    step_dependencies: {},
    step_outputs: HStore {
        map: {},
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::1461] &process: Popen {
    stdin: None,
    stdout: Some(
        File {
            fd: 5,
            read: true,
            write: false,
        },
    ),
    stderr: Some(
        File {
            fd: 7,
            read: true,
            write: false,
        },
    ),
    child_state: Running {
        pid: 21337,
        ext: (),
    },
    detached: true,
}
[TRACE][pipeline/src/pipeline/mod.rs::1466] process: Popen {
    stdin: None,
    stdout: Some(
        File {
            fd: 5,
            read: true,
            write: false,
        },
    ),
    stderr: Some(
        File {
            fd: 7,
            read: true,
            write: false,
        },
    ),
    child_state: Running {
        pid: 21337,
        ext: (),
    },
    detached: true,
}
[DEBUG] Step average-age with command sqlite3 people.db 'SELECT AVG(Age) FROM People;' is still running
[OUT] [average-age] 34.6666666666667

[TRACE][pipeline/src/pipeline/mod.rs::1461] &process: Popen {
    stdin: None,
    stdout: Some(
        File {
            fd: 5,
            read: true,
            write: false,
        },
    ),
    stderr: Some(
        File {
            fd: 7,
            read: true,
            write: false,
        },
    ),
    child_state: Running {
        pid: 21337,
        ext: (),
    },
    detached: true,
}
[DONE] average-age (sqlite3 people.db 'SELECT AVG(Age) FROM People;')
[TRACE][pipeline/src/pipeline/mod.rs::1513] return_state: Some(
    DoneByRunning(
        FromProcessCompletedSuccessfully,
    ),
)

[TRACE][pipeline/src/pipeline/mod.rs::1519] params: StepStateParams {
    xvc_root: XvcRootInner {
        absolute_path: AbsolutePath(
            "[CWD]",
        ),
        xvc_dir: AbsolutePath(
            "[CWD]/.xvc",
        ),
        store_dir: AbsolutePath(
            "[CWD]/.xvc/store",
        ),
        config: XvcConfig {
            current_dir: XvcConfigOption {
                source: Runtime,
                option: AbsolutePath(
                    "[CWD]",
                ),
            },
            config_maps: [
                XvcConfigMap {
                    source: Default,
                    map: {
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "core.verbosity": String(
                            "error",
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "file.list.sort": String(
                            "name-desc",
                        ),
                        "git.use_git": Boolean(
                            true,
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "core.guid": String(
                            "919ce8649f3e32f7",
                        ),
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "pipeline.process_pool_size": Integer(
                            4,
                        ),
                        "git.command": String(
                            "git",
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "file.list.show_dot_files": Boolean(
                            false,
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                        "file.list.no_summary": Boolean(
                            false,
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                    },
                },
                XvcConfigMap {
                    source: Project,
                    map: {
                        "core.guid": String(
                            "59a2d95849757118",
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "git.command": String(
                            "git",
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "git.use_git": Boolean(
                            true,
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "file.list.no_summary": Boolean(
                            false,
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "file.list.show_dot_files": Boolean(
                            false,
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "file.list.sort": String(
                            "name-desc",
                        ),
                        "pipeline.process_pool_size": Integer(
                            4,
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "core.verbosity": String(
                            "error",
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                    },
                },
                XvcConfigMap {
                    source: Local,
                    map: {},
                },
                XvcConfigMap {
                    source: Environment,
                    map: {
                        "TRYCMD_TESTS": String(
                            "storage,file,pipeline,core",
                        ),
                    },
                },
                XvcConfigMap {
                    source: CommandLine,
                    map: {
                        "core.quiet": Boolean(
                            false,
                        ),
                        "core.verbosity": String(
                            "debug",
                        ),
                    },
                },
            ],
            the_config: {
                "pipeline.default_params_file": XvcConfigValue {
                    source: Project,
                    value: String(
                        "params.yaml",
                    ),
                },
                "file.list.sort": XvcConfigValue {
                    source: Project,
                    value: String(
                        "name-desc",
                    ),
                },
                "git.use_git": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        true,
                    ),
                },
                "git.auto_commit": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        true,
                    ),
                },
                "core.verbosity": XvcConfigValue {
                    source: CommandLine,
                    value: String(
                        "debug",
                    ),
                },
                "git.auto_stage": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "file.carry-in.force": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "pipeline.current_pipeline": XvcConfigValue {
                    source: Project,
                    value: String(
                        "default",
                    ),
                },
                "cache.algorithm": XvcConfigValue {
                    source: Project,
                    value: String(
                        "blake3",
                    ),
                },
                "pipeline.process_pool_size": XvcConfigValue {
                    source: Project,
                    value: Integer(
                        4,
                    ),
                },
                "file.list.no_summary": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "file.track.no_parallel": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "file.recheck.method": XvcConfigValue {
                    source: Project,
                    value: String(
                        "copy",
                    ),
                },
                "core.quiet": XvcConfigValue {
                    source: CommandLine,
                    value: Boolean(
                        false,
                    ),
                },
                "file.carry-in.no_parallel": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "core.guid": XvcConfigValue {
                    source: Project,
                    value: String(
                        "59a2d95849757118",
                    ),
                },
                "pipeline.default": XvcConfigValue {
                    source: Project,
                    value: String(
                        "default",
                    ),
                },
                "git.command": XvcConfigValue {
                    source: Project,
                    value: String(
                        "git",
                    ),
                },
                "file.track.force": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "file.track.no_commit": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "file.list.recursive": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "file.list.format": XvcConfigValue {
                    source: Project,
                    value: String(
                        "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                    ),
                },
                "file.track.text_or_binary": XvcConfigValue {
                    source: Project,
                    value: String(
                        "auto",
                    ),
                },
                "TRYCMD_TESTS": XvcConfigValue {
                    source: Environment,
                    value: String(
                        "storage,file,pipeline,core",
                    ),
                },
                "file.list.show_dot_files": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
            },
            init_params: XvcConfigParams {
                default_configuration: "
[core]
# The repository id. Please do not delete or change it.
# This is used to identify the repository and generate paths in storages.
# In the future it may be used to in other ways.
guid = /"919ce8649f3e32f7/"
# Default verbosity level.
# One of /"error/", /"warn/", /"info/"
verbosity = /"error/"

[git]
# Automate git operations.
# Turning this off leads Xvc to behave as if it's not in a Git repository.
# Not recommended unless you're really not using Git
use_git = true
# Command to run Git process.
# You can set this to an absolute path to specify an executable
# If set to a non-absolute path, the executable will be searched in $PATH.
command = /"git/"

# Commit changes in .xvc/ directory after commands.
# You can set this to false if you want to commit manually.
auto_commit = true

# Stage changes in .xvc/ directory without committing.
# auto_commit implies auto_stage.
# If you want to commit manually but don't want to stage after individual Xvc commands, you can set this to true.
auto_stage = false

[cache]
# The hash algorithm used for the cache.
# It may take blake3, blake2, sha2 or sha3 as values.
# All algorithms are selected to produce 256-bit hashes, so sha2 means SHA2-256, blake2 means BLAKE2s, etc.
# The cache path is produced by prepending algorithm name to the cache.
# Blake3 files are in .xvc/b3/, while sha2 files are in .xvc/s2/ etc.
algorithm = /"blake3/"

[file]

[file.track]

# Don't move file content to cache after xvc file track
no_commit = false
# Force to track files even if they are already tracked.
force = false

# Xvc calculates file content digest differently for text and binary files.
# This option controls whether to treat files as text or binary.
# It may take auto, text or binary as values.
# Auto check each file individually and treat it as text if it's text.
text_or_binary = /"auto/"

# Don't use parallelism in track operations.
# Note that some of the operations are implemented in parallel by default, and this option affects some heavier operations.
no_parallel = false

[file.list]

# Format for `xvc file list` rows. You can reorder or remove columns.
# The following are the keys for each row:
# - {acd64}:  actual content digest. All 64 digits from the workspace file's content.
# - {acd8}:  actual content digest. First 8 digits the file content digest.
# - {aft}:  actual file type. Whether the entry is a file (F), directory (D),
#   symlink (S), hardlink (H) or reflink (R).
# - {asz}:  actual size. The size of the workspace file in bytes. It uses MB,
#   GB and TB to represent sizes larger than 1MB.
# - {ats}:  actual timestamp. The timestamp of the workspace file.
# - {cst}:  cache status. One of /"=/", /">/", /"</", /"X/", or /"?/" to show
#   whether the file timestamp is the same as the cached timestamp, newer,
#   older, not cached or not tracked.
# - {name}: The name of the file or directory.
# - {rcd64}:  recorded content digest. All 64 digits.
# - {rcd8}:  recorded content digest. First 8 digits.
# - {rrm}:  recorded recheck method. Whether the entry is linked to the workspace
#   as a copy (C), symlink (S), hardlink (H) or reflink (R).
# - {rsz}:  recorded size. The size of the cached content in bytes. It uses
#   MB, GB and TB to represent sizes larged than 1MB.
# - {rts}:  recorded timestamp. The timestamp of the cached content.
#
# There are no escape sequences in the format string.
# If you want to add a tab, type it to the string.
# If you want to add a literal double curly brace, open an issue.
format = /"{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}/"

# Default sort order for `xvc file list`.
# Valid values are
# none, name-asc, name-desc, size-asc, size-desc, ts-asc, ts-desc.
sort = /"name-desc/"

# Show dot files like .gitignore
show_dot_files = false

# Do not show a summary for as the final row for `xvc file list`.
no_summary = false

# List files recursively always.
recursive = false

[file.carry-in]
# Carry-in the files to cache always, even if they are already present.
force = false

# Don't use parallel move/copy in carry-in
no_parallel = false

[file.recheck]
# The recheck method for Xvc. It may take copy, hardlink, symlink, reflink as values.
# The default is copy to make sure the options is portable.
# Copy duplicates the file content, while hardlink, symlink and reflink only create a new path to the file.
# Note that hardlink and symlink are read-only as they link the files in cache.
method = /"copy/"

[pipeline]
# Name of the current pipeline to run
current_pipeline = /"default/"
# Name of the default pipeline
default = /"default/"
# Name of the default params file name
default_params_file = /"params.yaml/"
# Number of command processes to run concurrently
process_pool_size = 4
#

",
                current_dir: AbsolutePath(
                    "[CWD]",
                ),
                include_system_config: true,
                include_user_config: true,
                project_config_path: Some(
                    AbsolutePath(
                        "[CWD]/.xvc/config.toml",
                    ),
                ),
                local_config_path: Some(
                    AbsolutePath(
                        "[CWD]/.xvc/config.local.toml",
                    ),
                ),
                include_environment_config: true,
                command_line_config: Some(
                    [
                        "core.verbosity = debug",
                        "core.quiet = false",
                    ],
                ),
            },
        },
        local_config_path: AbsolutePath(
            "[CWD]/.xvc/config.local.toml",
        ),
        project_config_path: AbsolutePath(
            "[CWD]/.xvc/config.toml",
        ),
        entity_generator: XvcEntityGenerator {
            counter: 4,
            random: 13513277701724141613,
            dirty: false,
        },
    },
    output_snd: Sender { .. },
    pmp: XvcPathMetadataProvider {
        xvc_root: XvcRootInner {
            absolute_path: AbsolutePath(
                "[CWD]",
            ),
            xvc_dir: AbsolutePath(
                "[CWD]/.xvc",
            ),
            store_dir: AbsolutePath(
                "[CWD]/.xvc/store",
            ),
            config: XvcConfig {
                current_dir: XvcConfigOption {
                    source: Runtime,
                    option: AbsolutePath(
                        "[CWD]",
                    ),
                },
                config_maps: [
                    XvcConfigMap {
                        source: Default,
                        map: {
                            "pipeline.default_params_file": String(
                                "params.yaml",
                            ),
                            "file.carry-in.no_parallel": Boolean(
                                false,
                            ),
                            "file.carry-in.force": Boolean(
                                false,
                            ),
                            "core.verbosity": String(
                                "error",
                            ),
                            "file.recheck.method": String(
                                "copy",
                            ),
                            "file.list.sort": String(
                                "name-desc",
                            ),
                            "git.use_git": Boolean(
                                true,
                            ),
                            "pipeline.current_pipeline": String(
                                "default",
                            ),
                            "core.guid": String(
                                "919ce8649f3e32f7",
                            ),
                            "git.auto_commit": Boolean(
                                true,
                            ),
                            "pipeline.default": String(
                                "default",
                            ),
                            "pipeline.process_pool_size": Integer(
                                4,
                            ),
                            "git.command": String(
                                "git",
                            ),
                            "file.track.no_commit": Boolean(
                                false,
                            ),
                            "file.track.force": Boolean(
                                false,
                            ),
                            "file.list.show_dot_files": Boolean(
                                false,
                            ),
                            "file.list.recursive": Boolean(
                                false,
                            ),
                            "file.list.format": String(
                                "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                            ),
                            "cache.algorithm": String(
                                "blake3",
                            ),
                            "git.auto_stage": Boolean(
                                false,
                            ),
                            "file.list.no_summary": Boolean(
                                false,
                            ),
                            "file.track.text_or_binary": String(
                                "auto",
                            ),
                            "file.track.no_parallel": Boolean(
                                false,
                            ),
                        },
                    },
                    XvcConfigMap {
                        source: Project,
                        map: {
                            "core.guid": String(
                                "59a2d95849757118",
                            ),
                            "cache.algorithm": String(
                                "blake3",
                            ),
                            "file.list.recursive": Boolean(
                                false,
                            ),
                            "git.command": String(
                                "git",
                            ),
                            "pipeline.default_params_file": String(
                                "params.yaml",
                            ),
                            "file.track.force": Boolean(
                                false,
                            ),
                            "file.track.no_parallel": Boolean(
                                false,
                            ),
                            "git.use_git": Boolean(
                                true,
                            ),
                            "file.track.no_commit": Boolean(
                                false,
                            ),
                            "file.recheck.method": String(
                                "copy",
                            ),
                            "file.list.no_summary": Boolean(
                                false,
                            ),
                            "pipeline.current_pipeline": String(
                                "default",
                            ),
                            "git.auto_stage": Boolean(
                                false,
                            ),
                            "git.auto_commit": Boolean(
                                true,
                            ),
                            "file.list.show_dot_files": Boolean(
                                false,
                            ),
                            "file.list.format": String(
                                "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                            ),
                            "file.list.sort": String(
                                "name-desc",
                            ),
                            "pipeline.process_pool_size": Integer(
                                4,
                            ),
                            "file.carry-in.no_parallel": Boolean(
                                false,
                            ),
                            "pipeline.default": String(
                                "default",
                            ),
                            "file.carry-in.force": Boolean(
                                false,
                            ),
                            "core.verbosity": String(
                                "error",
                            ),
                            "file.track.text_or_binary": String(
                                "auto",
                            ),
                        },
                    },
                    XvcConfigMap {
                        source: Local,
                        map: {},
                    },
                    XvcConfigMap {
                        source: Environment,
                        map: {
                            "TRYCMD_TESTS": String(
                                "storage,file,pipeline,core",
                            ),
                        },
                    },
                    XvcConfigMap {
                        source: CommandLine,
                        map: {
                            "core.quiet": Boolean(
                                false,
                            ),
                            "core.verbosity": String(
                                "debug",
                            ),
                        },
                    },
                ],
                the_config: {
                    "pipeline.default_params_file": XvcConfigValue {
                        source: Project,
                        value: String(
                            "params.yaml",
                        ),
                    },
                    "file.list.sort": XvcConfigValue {
                        source: Project,
                        value: String(
                            "name-desc",
                        ),
                    },
                    "git.use_git": XvcConfigValue {
                        source: Project,
                        value: Boolean(
                            true,
                        ),
                    },
                    "git.auto_commit": XvcConfigValue {
                        source: Project,
                        value: Boolean(
                            true,
                        ),
                    },
                    "core.verbosity": XvcConfigValue {
                        source: CommandLine,
                        value: String(
                            "debug",
                        ),
                    },
                    "git.auto_stage": XvcConfigValue {
                        source: Project,
                        value: Boolean(
                            false,
                        ),
                    },
                    "file.carry-in.force": XvcConfigValue {
                        source: Project,
                        value: Boolean(
                            false,
                        ),
                    },
                    "pipeline.current_pipeline": XvcConfigValue {
                        source: Project,
                        value: String(
                            "default",
                        ),
                    },
                    "cache.algorithm": XvcConfigValue {
                        source: Project,
                        value: String(
                            "blake3",
                        ),
                    },
                    "pipeline.process_pool_size": XvcConfigValue {
                        source: Project,
                        value: Integer(
                            4,
                        ),
                    },
                    "file.list.no_summary": XvcConfigValue {
                        source: Project,
                        value: Boolean(
                            false,
                        ),
                    },
                    "file.track.no_parallel": XvcConfigValue {
                        source: Project,
                        value: Boolean(
                            false,
                        ),
                    },
                    "file.recheck.method": XvcConfigValue {
                        source: Project,
                        value: String(
                            "copy",
                        ),
                    },
                    "core.quiet": XvcConfigValue {
                        source: CommandLine,
                        value: Boolean(
                            false,
                        ),
                    },
                    "file.carry-in.no_parallel": XvcConfigValue {
                        source: Project,
                        value: Boolean(
                            false,
                        ),
                    },
                    "core.guid": XvcConfigValue {
                        source: Project,
                        value: String(
                            "59a2d95849757118",
                        ),
                    },
                    "pipeline.default": XvcConfigValue {
                        source: Project,
                        value: String(
                            "default",
                        ),
                    },
                    "git.command": XvcConfigValue {
                        source: Project,
                        value: String(
                            "git",
                        ),
                    },
                    "file.track.force": XvcConfigValue {
                        source: Project,
                        value: Boolean(
                            false,
                        ),
                    },
                    "file.track.no_commit": XvcConfigValue {
                        source: Project,
                        value: Boolean(
                            false,
                        ),
                    },
                    "file.list.recursive": XvcConfigValue {
                        source: Project,
                        value: Boolean(
                            false,
                        ),
                    },
                    "file.list.format": XvcConfigValue {
                        source: Project,
                        value: String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                    },
                    "file.track.text_or_binary": XvcConfigValue {
                        source: Project,
                        value: String(
                            "auto",
                        ),
                    },
                    "TRYCMD_TESTS": XvcConfigValue {
                        source: Environment,
                        value: String(
                            "storage,file,pipeline,core",
                        ),
                    },
                    "file.list.show_dot_files": XvcConfigValue {
                        source: Project,
                        value: Boolean(
                            false,
                        ),
                    },
                },
                init_params: XvcConfigParams {
                    default_configuration: "
[core]
# The repository id. Please do not delete or change it.
# This is used to identify the repository and generate paths in storages.
# In the future it may be used to in other ways.
guid = /"919ce8649f3e32f7/"
# Default verbosity level.
# One of /"error/", /"warn/", /"info/"
verbosity = /"error/"

[git]
# Automate git operations.
# Turning this off leads Xvc to behave as if it's not in a Git repository.
# Not recommended unless you're really not using Git
use_git = true
# Command to run Git process.
# You can set this to an absolute path to specify an executable
# If set to a non-absolute path, the executable will be searched in $PATH.
command = /"git/"

# Commit changes in .xvc/ directory after commands.
# You can set this to false if you want to commit manually.
auto_commit = true

# Stage changes in .xvc/ directory without committing.
# auto_commit implies auto_stage.
# If you want to commit manually but don't want to stage after individual Xvc commands, you can set this to true.
auto_stage = false

[cache]
# The hash algorithm used for the cache.
# It may take blake3, blake2, sha2 or sha3 as values.
# All algorithms are selected to produce 256-bit hashes, so sha2 means SHA2-256, blake2 means BLAKE2s, etc.
# The cache path is produced by prepending algorithm name to the cache.
# Blake3 files are in .xvc/b3/, while sha2 files are in .xvc/s2/ etc.
algorithm = /"blake3/"

[file]

[file.track]

# Don't move file content to cache after xvc file track
no_commit = false
# Force to track files even if they are already tracked.
force = false

# Xvc calculates file content digest differently for text and binary files.
# This option controls whether to treat files as text or binary.
# It may take auto, text or binary as values.
# Auto check each file individually and treat it as text if it's text.
text_or_binary = /"auto/"

# Don't use parallelism in track operations.
# Note that some of the operations are implemented in parallel by default, and this option affects some heavier operations.
no_parallel = false

[file.list]

# Format for `xvc file list` rows. You can reorder or remove columns.
# The following are the keys for each row:
# - {acd64}:  actual content digest. All 64 digits from the workspace file's content.
# - {acd8}:  actual content digest. First 8 digits the file content digest.
# - {aft}:  actual file type. Whether the entry is a file (F), directory (D),
#   symlink (S), hardlink (H) or reflink (R).
# - {asz}:  actual size. The size of the workspace file in bytes. It uses MB,
#   GB and TB to represent sizes larger than 1MB.
# - {ats}:  actual timestamp. The timestamp of the workspace file.
# - {cst}:  cache status. One of /"=/", /">/", /"</", /"X/", or /"?/" to show
#   whether the file timestamp is the same as the cached timestamp, newer,
#   older, not cached or not tracked.
# - {name}: The name of the file or directory.
# - {rcd64}:  recorded content digest. All 64 digits.
# - {rcd8}:  recorded content digest. First 8 digits.
# - {rrm}:  recorded recheck method. Whether the entry is linked to the workspace
#   as a copy (C), symlink (S), hardlink (H) or reflink (R).
# - {rsz}:  recorded size. The size of the cached content in bytes. It uses
#   MB, GB and TB to represent sizes larged than 1MB.
# - {rts}:  recorded timestamp. The timestamp of the cached content.
#
# There are no escape sequences in the format string.
# If you want to add a tab, type it to the string.
# If you want to add a literal double curly brace, open an issue.
format = /"{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}/"

# Default sort order for `xvc file list`.
# Valid values are
# none, name-asc, name-desc, size-asc, size-desc, ts-asc, ts-desc.
sort = /"name-desc/"

# Show dot files like .gitignore
show_dot_files = false

# Do not show a summary for as the final row for `xvc file list`.
no_summary = false

# List files recursively always.
recursive = false

[file.carry-in]
# Carry-in the files to cache always, even if they are already present.
force = false

# Don't use parallel move/copy in carry-in
no_parallel = false

[file.recheck]
# The recheck method for Xvc. It may take copy, hardlink, symlink, reflink as values.
# The default is copy to make sure the options is portable.
# Copy duplicates the file content, while hardlink, symlink and reflink only create a new path to the file.
# Note that hardlink and symlink are read-only as they link the files in cache.
method = /"copy/"

[pipeline]
# Name of the current pipeline to run
current_pipeline = /"default/"
# Name of the default pipeline
default = /"default/"
# Name of the default params file name
default_params_file = /"params.yaml/"
# Number of command processes to run concurrently
process_pool_size = 4
#

",
                    current_dir: AbsolutePath(
                        "[CWD]",
                    ),
                    include_system_config: true,
                    include_user_config: true,
                    project_config_path: Some(
                        AbsolutePath(
                            "[CWD]/.xvc/config.toml",
                        ),
                    ),
                    local_config_path: Some(
                        AbsolutePath(
                            "[CWD]/.xvc/config.local.toml",
                        ),
                    ),
                    include_environment_config: true,
                    command_line_config: Some(
                        [
                            "core.verbosity = debug",
                            "core.quiet = false",
                        ],
                    ),
                },
            },
            local_config_path: AbsolutePath(
                "[CWD]/.xvc/config.local.toml",
            ),
            project_config_path: AbsolutePath(
                "[CWD]/.xvc/config.toml",
            ),
            entity_generator: XvcEntityGenerator {
                counter: 4,
                random: 13513277701724141613,
                dirty: false,
            },
        },
        path_map: RwLock {
            data: {
                XvcPath(
                    "people.db",
                ): XvcMetadata {
                    file_type: File,
                    size: Some(
                        8192,
                    ),
                    modified: Some(
                        SystemTime {
                            tv_sec: 1722024927,
                            tv_nsec: 262703931,
                        },
                    ),
                },
            },
            poisoned: false,
            ..
        },
        kill_signal_sender: Sender { .. },
        background_thread: Mutex {
            data: JoinHandle { .. },
            poisoned: false,
            ..
        },
        output_sender: Sender { .. },
        ignore_rules: IgnoreRules {
            root: "[CWD]",
            ignore_patterns: RwLock {
                data: [
                    Pattern {
                        pattern: Glob {
                            glob: "**/.xvc",
                            re: "(?-u)^(?:/?|.*/)//.xvc$",
                            opts: GlobOptions {
                                case_insensitive: false,
                                literal_separator: false,
                                backslash_escape: true,
                                empty_alternates: false,
                            },
                            tokens: Tokens(
                                [
                                    RecursivePrefix,
                                    Literal(
                                        '.',
                                    ),
                                    Literal(
                                        'x',
                                    ),
                                    Literal(
                                        'v',
                                    ),
                                    Literal(
                                        'c',
                                    ),
                                ],
                            ),
                        },
                        original: ".xvc",
                        source: Global,
                        effect: Ignore,
                        relativity: Anywhere,
                        path_kind: Any,
                    },
                    Pattern {
                        pattern: Glob {
                            glob: "**/.git",
                            re: "(?-u)^(?:/?|.*/)//.git$",
                            opts: GlobOptions {
                                case_insensitive: false,
                                literal_separator: false,
                                backslash_escape: true,
                                empty_alternates: false,
                            },
                            tokens: Tokens(
                                [
                                    RecursivePrefix,
                                    Literal(
                                        '.',
                                    ),
                                    Literal(
                                        'g',
                                    ),
                                    Literal(
                                        'i',
                                    ),
                                    Literal(
                                        't',
                                    ),
                                ],
                            ),
                        },
                        original: ".git",
                        source: Global,
                        effect: Ignore,
                        relativity: Anywhere,
                        path_kind: Any,
                    },
                    Pattern {
                        pattern: Glob {
                            glob: "**/.DS_Store",
                            re: "(?-u)^(?:/?|.*/)//.DS_Store$",
                            opts: GlobOptions {
                                case_insensitive: false,
                                literal_separator: false,
                                backslash_escape: true,
                                empty_alternates: false,
                            },
                            tokens: Tokens(
                                [
                                    RecursivePrefix,
                                    Literal(
                                        '.',
                                    ),
                                    Literal(
                                        'D',
                                    ),
                                    Literal(
                                        'S',
                                    ),
                                    Literal(
                                        '_',
                                    ),
                                    Literal(
                                        'S',
                                    ),
                                    Literal(
                                        't',
                                    ),
                                    Literal(
                                        'o',
                                    ),
                                    Literal(
                                        'r',
                                    ),
                                    Literal(
                                        'e',
                                    ),
                                ],
                            ),
                        },
                        original: ".DS_Store",
                        source: File {
                            path: ".xvcignore",
                            line: 6,
                        },
                        effect: Ignore,
                        relativity: Anywhere,
                        path_kind: Any,
                    },
                ],
                poisoned: false,
                ..
            },
            whitelist_patterns: RwLock {
                data: [],
                poisoned: false,
                ..
            },
            whitelist_set: RwLock {
                data: GlobSet {
                    len: 0,
                    strats: [],
                },
                poisoned: false,
                ..
            },
            ignore_set: RwLock {
                data: GlobSet {
                    len: 3,
                    strats: [
                        Extension(
                            ExtensionStrategy(
                                {},
                            ),
                        ),
                        BasenameLiteral(
                            BasenameLiteralStrategy(
                                {
                                    [
                                        46,
                                        120,
                                        118,
                                        99,
                                    ]: [
                                        0,
                                    ],
                                    [
                                        46,
                                        68,
                                        83,
                                        95,
                                        83,
                                        116,
                                        111,
                                        114,
                                        101,
                                    ]: [
                                        2,
                                    ],
                                    [
                                        46,
                                        103,
                                        105,
                                        116,
                                    ]: [
                                        1,
                                    ],
                                },
                            ),
                        ),
                        Literal(
                            LiteralStrategy(
                                {},
                            ),
                        ),
                        Suffix(
                            SuffixStrategy {
                                matcher: AhoCorasick(
                                    dfa::DFA(
                                    D 000000: /x00 => 0
                                    F 000001:
                                     >000002: /x00 => 2
                                      000003: /x00 => 0
                                    match kind: Standard
                                    prefilter: false
                                    state length: 4
                                    pattern length: 0
                                    shortest pattern length: 18446744073709551615
                                    longest pattern length: 0
                                    alphabet length: 1
                                    stride: 1
                                    byte classes: ByteClasses(0 => [0-255])
                                    memory usage: 16
                                    )
                                    ,
                                ),
                                map: [],
                                longest: 0,
                            },
                        ),
                        Prefix(
                            PrefixStrategy {
                                matcher: AhoCorasick(
                                    dfa::DFA(
                                    D 000000: /x00 => 0
                                    F 000001:
                                     >000002: /x00 => 2
                                      000003: /x00 => 0
                                    match kind: Standard
                                    prefilter: false
                                    state length: 4
                                    pattern length: 0
                                    shortest pattern length: 18446744073709551615
                                    longest pattern length: 0
                                    alphabet length: 1
                                    stride: 1
                                    byte classes: ByteClasses(0 => [0-255])
                                    memory usage: 16
                                    )
                                    ,
                                ),
                                map: [],
                                longest: 0,
                            },
                        ),
                        RequiredExtension(
                            RequiredExtensionStrategy(
                                {},
                            ),
                        ),
                        Regex(
                            RegexSetStrategy {
                                matcher: Regex {
                                    imp: RegexI {
                                        strat: Core {
                                            info: RegexInfo(
                                                RegexInfoI {
                                                    config: Config {
                                                        match_kind: Some(
                                                            All,
                                                        ),
                                                        utf8_empty: Some(
                                                            false,
                                                        ),
                                                        autopre: None,
                                                        pre: None,
                                                        which_captures: None,
                                                        nfa_size_limit: Some(
                                                            Some(
                                                                10485760,
                                                            ),
                                                        ),
                                                        onepass_size_limit: None,
                                                        hybrid_cache_capacity: Some(
                                                            10485760,
                                                        ),
                                                        hybrid: None,
                                                        dfa: None,
                                                        dfa_size_limit: None,
                                                        dfa_state_limit: None,
                                                        onepass: None,
                                                        backtrack: None,
                                                        byte_classes: None,
                                                        line_terminator: None,
                                                    },
                                                    props: [],
                                                    props_union: Properties(
                                                        PropertiesI {
                                                            minimum_len: None,
                                                            maximum_len: None,
                                                            look_set: ∅,
                                                            look_set_prefix: ∅,
                                                            look_set_suffix: ∅,
                                                            look_set_prefix_any: ∅,
                                                            look_set_suffix_any: ∅,
                                                            utf8: true,
                                                            explicit_captures_len: 0,
                                                            static_explicit_captures_len: None,
                                                            literal: false,
                                                            alternation_literal: true,
                                                        },
                                                    ),
                                                },
                                            ),
                                            pre: None,
                                            nfa: thompson::NFA(
                                            ^000000: FAIL

                                            transition equivalence classes: ByteClasses(0 => [/x00-/xFF], 1 => [EOI])
                                            )
                                            ,
                                            nfarev: Some(
                                                thompson::NFA(
                                                ^000000: FAIL

                                                transition equivalence classes: ByteClasses(0 => [/x00-/xFF], 1 => [EOI])
                                                )
                                                ,
                                            ),
                                            pikevm: PikeVM(
                                                PikeVMEngine(
                                                    PikeVM {
                                                        config: Config {
                                                            match_kind: Some(
                                                                All,
                                                            ),
                                                            pre: Some(
                                                                None,
                                                            ),
                                                        },
                                                        nfa: thompson::NFA(
                                                        ^000000: FAIL

                                                        transition equivalence classes: ByteClasses(0 => [/x00-/xFF], 1 => [EOI])
                                                        )
                                                        ,
                                                    },
                                                ),
                                            ),
                                            backtrack: BoundedBacktracker(
                                                None,
                                            ),
                                            onepass: OnePass(
                                                None,
                                            ),
                                            hybrid: Hybrid(
                                                Some(
                                                    HybridEngine(
                                                        Regex {
                                                            forward: DFA {
                                                                config: Config {
                                                                    match_kind: Some(
                                                                        All,
                                                                    ),
                                                                    pre: Some(
                                                                        None,
                                                                    ),
                                                                    starts_for_each_pattern: Some(
                                                                        true,
                                                                    ),
                                                                    byte_classes: Some(
                                                                        true,
                                                                    ),
                                                                    unicode_word_boundary: Some(
                                                                        true,
                                                                    ),
                                                                    quitset: None,
                                                                    specialize_start_states: Some(
                                                                        false,
                                                                    ),
                                                                    cache_capacity: Some(
                                                                        10485760,
                                                                    ),
                                                                    skip_cache_capacity_check: Some(
                                                                        false,
                                                                    ),
                                                                    minimum_cache_clear_count: Some(
                                                                        Some(
                                                                            3,
                                                                        ),
                                                                    ),
                                                                    minimum_bytes_per_state: Some(
                                                                        Some(
                                                                            10,
                                                                        ),
                                                                    ),
                                                                },
                                                                nfa: thompson::NFA(
                                                                ^000000: FAIL

                                                                transition equivalence classes: ByteClasses(0 => [/x00-/xFF], 1 => [EOI])
                                                                )
                                                                ,
                                                                stride2: 1,
                                                                start_map: StartByteMap{/x00 => NonWordByte, /x01 => NonWordByte, /x02 => NonWordByte, /x03 => NonWordByte, /x04 => NonWordByte, /x05 => NonWordByte, /x06 => NonWordByte, /x07 => NonWordByte, /x08 => NonWordByte, /t => NonWordByte,
 => LineLF, /x0B => NonWordByte, /x0C => NonWordByte, /r => LineCR, /x0E => NonWordByte, /x0F => NonWordByte, /x10 => NonWordByte, /x11 => NonWordByte, /x12 => NonWordByte, /x13 => NonWordByte, /x14 => NonWordByte, /x15 => NonWordByte, /x16 => NonWordByte, /x17 => NonWordByte, /x18 => NonWordByte, /x19 => NonWordByte, /x1A => NonWordByte, /x1B => NonWordByte, /x1C => NonWordByte, /x1D => NonWordByte, /x1E => NonWordByte, /x1F => NonWordByte, ' ' => NonWordByte, ! => NonWordByte, /" => NonWordByte, # => NonWordByte, $ => NonWordByte, % => NonWordByte, & => NonWordByte, /' => NonWordByte, ( => NonWordByte, ) => NonWordByte, * => NonWordByte, + => NonWordByte, , => NonWordByte, - => NonWordByte, . => NonWordByte, / => NonWordByte, 0 => WordByte, 1 => WordByte, 2 => WordByte, 3 => WordByte, 4 => WordByte, 5 => WordByte, 6 => WordByte, 7 => WordByte, 8 => WordByte, 9 => WordByte, : => NonWordByte, ; => NonWordByte, < => NonWordByte, = => NonWordByte, > => NonWordByte, ? => NonWordByte, @ => NonWordByte, A => WordByte, B => WordByte, C => WordByte, D => WordByte, E => WordByte, F => WordByte, G => WordByte, H => WordByte, I => WordByte, J => WordByte, K => WordByte, L => WordByte, M => WordByte, N => WordByte, O => WordByte, P => WordByte, Q => WordByte, R => WordByte, S => WordByte, T => WordByte, U => WordByte, V => WordByte, W => WordByte, X => WordByte, Y => WordByte, Z => WordByte, [ => NonWordByte, // => NonWordByte, ] => NonWordByte, ^ => NonWordByte, _ => WordByte, ` => NonWordByte, a => WordByte, b => WordByte, c => WordByte, d => WordByte, e => WordByte, f => WordByte, g => WordByte, h => WordByte, i => WordByte, j => WordByte, k => WordByte, l => WordByte, m => WordByte, n => WordByte, o => WordByte, p => WordByte, q => WordByte, r => WordByte, s => WordByte, t => WordByte, u => WordByte, v => WordByte, w => WordByte, x => WordByte, y => WordByte, z => WordByte, { => NonWordByte, | => NonWordByte, } => NonWordByte, ~ => NonWordByte, /x7F => NonWordByte, /x80 => NonWordByte, /x81 => NonWordByte, /x82 => NonWordByte, /x83 => NonWordByte, /x84 => NonWordByte, /x85 => NonWordByte, /x86 => NonWordByte, /x87 => NonWordByte, /x88 => NonWordByte, /x89 => NonWordByte, /x8A => NonWordByte, /x8B => NonWordByte, /x8C => NonWordByte, /x8D => NonWordByte, /x8E => NonWordByte, /x8F => NonWordByte, /x90 => NonWordByte, /x91 => NonWordByte, /x92 => NonWordByte, /x93 => NonWordByte, /x94 => NonWordByte, /x95 => NonWordByte, /x96 => NonWordByte, /x97 => NonWordByte, /x98 => NonWordByte, /x99 => NonWordByte, /x9A => NonWordByte, /x9B => NonWordByte, /x9C => NonWordByte, /x9D => NonWordByte, /x9E => NonWordByte, /x9F => NonWordByte, /xA0 => NonWordByte, /xA1 => NonWordByte, /xA2 => NonWordByte, /xA3 => NonWordByte, /xA4 => NonWordByte, /xA5 => NonWordByte, /xA6 => NonWordByte, /xA7 => NonWordByte, /xA8 => NonWordByte, /xA9 => NonWordByte, /xAA => NonWordByte, /xAB => NonWordByte, /xAC => NonWordByte, /xAD => NonWordByte, /xAE => NonWordByte, /xAF => NonWordByte, /xB0 => NonWordByte, /xB1 => NonWordByte, /xB2 => NonWordByte, /xB3 => NonWordByte, /xB4 => NonWordByte, /xB5 => NonWordByte, /xB6 => NonWordByte, /xB7 => NonWordByte, /xB8 => NonWordByte, /xB9 => NonWordByte, /xBA => NonWordByte, /xBB => NonWordByte, /xBC => NonWordByte, /xBD => NonWordByte, /xBE => NonWordByte, /xBF => NonWordByte, /xC0 => NonWordByte, /xC1 => NonWordByte, /xC2 => NonWordByte, /xC3 => NonWordByte, /xC4 => NonWordByte, /xC5 => NonWordByte, /xC6 => NonWordByte, /xC7 => NonWordByte, /xC8 => NonWordByte, /xC9 => NonWordByte, /xCA => NonWordByte, /xCB => NonWordByte, /xCC => NonWordByte, /xCD => NonWordByte, /xCE => NonWordByte, /xCF => NonWordByte, /xD0 => NonWordByte, /xD1 => NonWordByte, /xD2 => NonWordByte, /xD3 => NonWordByte, /xD4 => NonWordByte, /xD5 => NonWordByte, /xD6 => NonWordByte, /xD7 => NonWordByte, /xD8 => NonWordByte, /xD9 => NonWordByte, /xDA => NonWordByte, /xDB => NonWordByte, /xDC => NonWordByte, /xDD => NonWordByte, /xDE => NonWordByte, /xDF => NonWordByte, /xE0 => NonWordByte, /xE1 => NonWordByte, /xE2 => NonWordByte, /xE3 => NonWordByte, /xE4 => NonWordByte, /xE5 => NonWordByte, /xE6 => NonWordByte, /xE7 => NonWordByte, /xE8 => NonWordByte, /xE9 => NonWordByte, /xEA => NonWordByte, /xEB => NonWordByte, /xEC => NonWordByte, /xED => NonWordByte, /xEE => NonWordByte, /xEF => NonWordByte, /xF0 => NonWordByte, /xF1 => NonWordByte, /xF2 => NonWordByte, /xF3 => NonWordByte, /xF4 => NonWordByte, /xF5 => NonWordByte, /xF6 => NonWordByte, /xF7 => NonWordByte, /xF8 => NonWordByte, /xF9 => NonWordByte, /xFA => NonWordByte, /xFB => NonWordByte, /xFC => NonWordByte, /xFD => NonWordByte, /xFE => NonWordByte, /xFF => NonWordByte},
                                                                classes: ByteClasses(0 => [/x00-/xFF], 1 => [EOI]),
                                                                quitset: ByteSet {
                                                                    bits: {},
                                                                },
                                                                cache_capacity: 10485760,
                                                            },
                                                            reverse: DFA {
                                                                config: Config {
                                                                    match_kind: Some(
                                                                        All,
                                                                    ),
                                                                    pre: Some(
                                                                        None,
                                                                    ),
                                                                    starts_for_each_pattern: Some(
                                                                        true,
                                                                    ),
                                                                    byte_classes: Some(
                                                                        true,
                                                                    ),
                                                                    unicode_word_boundary: Some(
                                                                        true,
                                                                    ),
                                                                    quitset: None,
                                                                    specialize_start_states: Some(
                                                                        false,
                                                                    ),
                                                                    cache_capacity: Some(
                                                                        10485760,
                                                                    ),
                                                                    skip_cache_capacity_check: Some(
                                                                        false,
                                                                    ),
                                                                    minimum_cache_clear_count: Some(
                                                                        Some(
                                                                            3,
                                                                        ),
                                                                    ),
                                                                    minimum_bytes_per_state: Some(
                                                                        Some(
                                                                            10,
                                                                        ),
                                                                    ),
                                                                },
                                                                nfa: thompson::NFA(
                                                                ^000000: FAIL

                                                                transition equivalence classes: ByteClasses(0 => [/x00-/xFF], 1 => [EOI])
                                                                )
                                                                ,
                                                                stride2: 1,
                                                                start_map: StartByteMap{/x00 => NonWordByte, /x01 => NonWordByte, /x02 => NonWordByte, /x03 => NonWordByte, /x04 => NonWordByte, /x05 => NonWordByte, /x06 => NonWordByte, /x07 => NonWordByte, /x08 => NonWordByte, /t => NonWordByte,
 => LineLF, /x0B => NonWordByte, /x0C => NonWordByte, /r => LineCR, /x0E => NonWordByte, /x0F => NonWordByte, /x10 => NonWordByte, /x11 => NonWordByte, /x12 => NonWordByte, /x13 => NonWordByte, /x14 => NonWordByte, /x15 => NonWordByte, /x16 => NonWordByte, /x17 => NonWordByte, /x18 => NonWordByte, /x19 => NonWordByte, /x1A => NonWordByte, /x1B => NonWordByte, /x1C => NonWordByte, /x1D => NonWordByte, /x1E => NonWordByte, /x1F => NonWordByte, ' ' => NonWordByte, ! => NonWordByte, /" => NonWordByte, # => NonWordByte, $ => NonWordByte, % => NonWordByte, & => NonWordByte, /' => NonWordByte, ( => NonWordByte, ) => NonWordByte, * => NonWordByte, + => NonWordByte, , => NonWordByte, - => NonWordByte, . => NonWordByte, / => NonWordByte, 0 => WordByte, 1 => WordByte, 2 => WordByte, 3 => WordByte, 4 => WordByte, 5 => WordByte, 6 => WordByte, 7 => WordByte, 8 => WordByte, 9 => WordByte, : => NonWordByte, ; => NonWordByte, < => NonWordByte, = => NonWordByte, > => NonWordByte, ? => NonWordByte, @ => NonWordByte, A => WordByte, B => WordByte, C => WordByte, D => WordByte, E => WordByte, F => WordByte, G => WordByte, H => WordByte, I => WordByte, J => WordByte, K => WordByte, L => WordByte, M => WordByte, N => WordByte, O => WordByte, P => WordByte, Q => WordByte, R => WordByte, S => WordByte, T => WordByte, U => WordByte, V => WordByte, W => WordByte, X => WordByte, Y => WordByte, Z => WordByte, [ => NonWordByte, // => NonWordByte, ] => NonWordByte, ^ => NonWordByte, _ => WordByte, ` => NonWordByte, a => WordByte, b => WordByte, c => WordByte, d => WordByte, e => WordByte, f => WordByte, g => WordByte, h => WordByte, i => WordByte, j => WordByte, k => WordByte, l => WordByte, m => WordByte, n => WordByte, o => WordByte, p => WordByte, q => WordByte, r => WordByte, s => WordByte, t => WordByte, u => WordByte, v => WordByte, w => WordByte, x => WordByte, y => WordByte, z => WordByte, { => NonWordByte, | => NonWordByte, } => NonWordByte, ~ => NonWordByte, /x7F => NonWordByte, /x80 => NonWordByte, /x81 => NonWordByte, /x82 => NonWordByte, /x83 => NonWordByte, /x84 => NonWordByte, /x85 => NonWordByte, /x86 => NonWordByte, /x87 => NonWordByte, /x88 => NonWordByte, /x89 => NonWordByte, /x8A => NonWordByte, /x8B => NonWordByte, /x8C => NonWordByte, /x8D => NonWordByte, /x8E => NonWordByte, /x8F => NonWordByte, /x90 => NonWordByte, /x91 => NonWordByte, /x92 => NonWordByte, /x93 => NonWordByte, /x94 => NonWordByte, /x95 => NonWordByte, /x96 => NonWordByte, /x97 => NonWordByte, /x98 => NonWordByte, /x99 => NonWordByte, /x9A => NonWordByte, /x9B => NonWordByte, /x9C => NonWordByte, /x9D => NonWordByte, /x9E => NonWordByte, /x9F => NonWordByte, /xA0 => NonWordByte, /xA1 => NonWordByte, /xA2 => NonWordByte, /xA3 => NonWordByte, /xA4 => NonWordByte, /xA5 => NonWordByte, /xA6 => NonWordByte, /xA7 => NonWordByte, /xA8 => NonWordByte, /xA9 => NonWordByte, /xAA => NonWordByte, /xAB => NonWordByte, /xAC => NonWordByte, /xAD => NonWordByte, /xAE => NonWordByte, /xAF => NonWordByte, /xB0 => NonWordByte, /xB1 => NonWordByte, /xB2 => NonWordByte, /xB3 => NonWordByte, /xB4 => NonWordByte, /xB5 => NonWordByte, /xB6 => NonWordByte, /xB7 => NonWordByte, /xB8 => NonWordByte, /xB9 => NonWordByte, /xBA => NonWordByte, /xBB => NonWordByte, /xBC => NonWordByte, /xBD => NonWordByte, /xBE => NonWordByte, /xBF => NonWordByte, /xC0 => NonWordByte, /xC1 => NonWordByte, /xC2 => NonWordByte, /xC3 => NonWordByte, /xC4 => NonWordByte, /xC5 => NonWordByte, /xC6 => NonWordByte, /xC7 => NonWordByte, /xC8 => NonWordByte, /xC9 => NonWordByte, /xCA => NonWordByte, /xCB => NonWordByte, /xCC => NonWordByte, /xCD => NonWordByte, /xCE => NonWordByte, /xCF => NonWordByte, /xD0 => NonWordByte, /xD1 => NonWordByte, /xD2 => NonWordByte, /xD3 => NonWordByte, /xD4 => NonWordByte, /xD5 => NonWordByte, /xD6 => NonWordByte, /xD7 => NonWordByte, /xD8 => NonWordByte, /xD9 => NonWordByte, /xDA => NonWordByte, /xDB => NonWordByte, /xDC => NonWordByte, /xDD => NonWordByte, /xDE => NonWordByte, /xDF => NonWordByte, /xE0 => NonWordByte, /xE1 => NonWordByte, /xE2 => NonWordByte, /xE3 => NonWordByte, /xE4 => NonWordByte, /xE5 => NonWordByte, /xE6 => NonWordByte, /xE7 => NonWordByte, /xE8 => NonWordByte, /xE9 => NonWordByte, /xEA => NonWordByte, /xEB => NonWordByte, /xEC => NonWordByte, /xED => NonWordByte, /xEE => NonWordByte, /xEF => NonWordByte, /xF0 => NonWordByte, /xF1 => NonWordByte, /xF2 => NonWordByte, /xF3 => NonWordByte, /xF4 => NonWordByte, /xF5 => NonWordByte, /xF6 => NonWordByte, /xF7 => NonWordByte, /xF8 => NonWordByte, /xF9 => NonWordByte, /xFA => NonWordByte, /xFB => NonWordByte, /xFC => NonWordByte, /xFD => NonWordByte, /xFE => NonWordByte, /xFF => NonWordByte},
                                                                classes: ByteClasses(0 => [/x00-/xFF], 1 => [EOI]),
                                                                quitset: ByteSet {
                                                                    bits: {},
                                                                },
                                                                cache_capacity: 10485760,
                                                            },
                                                        },
                                                    ),
                                                ),
                                            ),
                                            dfa: DFA(
                                                None,
                                            ),
                                        },
                                        info: RegexInfo(
                                            RegexInfoI {
                                                config: Config {
                                                    match_kind: Some(
                                                        All,
                                                    ),
                                                    utf8_empty: Some(
                                                        false,
                                                    ),
                                                    autopre: None,
                                                    pre: None,
                                                    which_captures: None,
                                                    nfa_size_limit: Some(
                                                        Some(
                                                            10485760,
                                                        ),
                                                    ),
                                                    onepass_size_limit: None,
                                                    hybrid_cache_capacity: Some(
                                                        10485760,
                                                    ),
                                                    hybrid: None,
                                                    dfa: None,
                                                    dfa_size_limit: None,
                                                    dfa_state_limit: None,
                                                    onepass: None,
                                                    backtrack: None,
                                                    byte_classes: None,
                                                    line_terminator: None,
                                                },
                                                props: [],
                                                props_union: Properties(
                                                    PropertiesI {
                                                        minimum_len: None,
                                                        maximum_len: None,
                                                        look_set: ∅,
                                                        look_set_prefix: ∅,
                                                        look_set_suffix: ∅,
                                                        look_set_prefix_any: ∅,
                                                        look_set_suffix_any: ∅,
                                                        utf8: true,
                                                        explicit_captures_len: 0,
                                                        static_explicit_captures_len: None,
                                                        literal: false,
                                                        alternation_literal: true,
                                                    },
                                                ),
                                            },
                                        ),
                                    },
                                    pool: Pool(
                                        Pool {
                                            stacks: [
                                                CacheLine(
                                                    Mutex {
                                                        data: [],
                                                        poisoned: false,
                                                        ..
                                                    },
                                                ),
                                                CacheLine(
                                                    Mutex {
                                                        data: [],
                                                        poisoned: false,
                                                        ..
                                                    },
                                                ),
                                                CacheLine(
                                                    Mutex {
                                                        data: [],
                                                        poisoned: false,
                                                        ..
                                                    },
                                                ),
                                                CacheLine(
                                                    Mutex {
                                                        data: [],
                                                        poisoned: false,
                                                        ..
                                                    },
                                                ),
                                                CacheLine(
                                                    Mutex {
                                                        data: [],
                                                        poisoned: false,
                                                        ..
                                                    },
                                                ),
                                                CacheLine(
                                                    Mutex {
                                                        data: [],
                                                        poisoned: false,
                                                        ..
                                                    },
                                                ),
                                                CacheLine(
                                                    Mutex {
                                                        data: [],
                                                        poisoned: false,
                                                        ..
                                                    },
                                                ),
                                                CacheLine(
                                                    Mutex {
                                                        data: [],
                                                        poisoned: false,
                                                        ..
                                                    },
                                                ),
                                            ],
                                            owner: 0,
                                            owner_val: UnsafeCell { .. },
                                        },
                                    ),
                                },
                                map: [],
                                patset: Pool(
                                    Pool {
                                        stacks: [
                                            CacheLine(
                                                Mutex {
                                                    data: [],
                                                    poisoned: false,
                                                    ..
                                                },
                                            ),
                                            CacheLine(
                                                Mutex {
                                                    data: [],
                                                    poisoned: false,
                                                    ..
                                                },
                                            ),
                                            CacheLine(
                                                Mutex {
                                                    data: [],
                                                    poisoned: false,
                                                    ..
                                                },
                                            ),
                                            CacheLine(
                                                Mutex {
                                                    data: [],
                                                    poisoned: false,
                                                    ..
                                                },
                                            ),
                                            CacheLine(
                                                Mutex {
                                                    data: [],
                                                    poisoned: false,
                                                    ..
                                                },
                                            ),
                                            CacheLine(
                                                Mutex {
                                                    data: [],
                                                    poisoned: false,
                                                    ..
                                                },
                                            ),
                                            CacheLine(
                                                Mutex {
                                                    data: [],
                                                    poisoned: false,
                                                    ..
                                                },
                                            ),
                                            CacheLine(
                                                Mutex {
                                                    data: [],
                                                    poisoned: false,
                                                    ..
                                                },
                                            ),
                                        ],
                                        owner: 0,
                                        owner_val: UnsafeCell { .. },
                                    },
                                ),
                            },
                        ),
                    ],
                },
                poisoned: false,
                ..
            },
        },
    },
    run_conditions: RunConditions {
        never: false,
        always: false,
        ignore_broken_dep_steps: false,
        ignore_missing_outputs: true,
    },
    pipeline_rundir: XvcPath(
        "",
    ),
    terminate_timeout_processes: true,
    algorithm: Blake3,
    command_process: RwLock {
        data: CommandProcess {
            environment: {},
            step: XvcStep {
                name: "average-age",
            },
            step_command: XvcStepCommand {
                command: "sqlite3 people.db 'SELECT AVG(Age) FROM People;'",
            },
            birth: Some(
                Instant {
                    tv_sec: 834347,
                    tv_nsec: 663124791,
                },
            ),
            process: Some(
                Popen {
                    stdin: None,
                    stdout: Some(
                        File {
                            fd: 5,
                            read: true,
                            write: false,
                        },
                    ),
                    stderr: Some(
                        File {
                            fd: 7,
                            read: true,
                            write: false,
                        },
                    ),
                    child_state: Finished(
                        Exited(
                            0,
                        ),
                    ),
                    detached: true,
                },
            ),
            stdout_sender: Sender { .. },
            stderr_sender: Sender { .. },
            stdout_receiver: Receiver { .. },
            stderr_receiver: Receiver { .. },
        },
        poisoned: false,
        ..
    },
    available_process_slots: RwLock {
        data: <locked>,
        poisoned: false,
        ..
    },
    process_poll_milliseconds: 10,
    dependency_diffs: RwLock {
        data: HStore {
            map: {
                XvcEntity(
                    3,
                    9022053985516504033,
                ): RecordMissing {
                    actual: SqliteQueryDigest(
                        SqliteQueryDep {
                            path: XvcPath(
                                "people.db",
                            ),
                            query: "SELECT count(*) FROM People;",
                            query_digest: Some(
                                ContentDigest(
                                    XvcDigest {
                                        algorithm: Blake3,
                                        digest: [
                                            11,
                                            225,
                                            214,
                                            145,
                                            174,
                                            151,
                                            99,
                                            217,
                                            216,
                                            197,
                                            211,
                                            26,
                                            216,
                                            218,
                                            115,
                                            209,
                                            161,
                                            95,
                                            15,
                                            52,
                                            174,
                                            24,
                                            193,
                                            209,
                                            218,
                                            91,
                                            154,
                                            207,
                                            247,
                                            217,
                                            245,
                                            9,
                                        ],
                                    },
                                ),
                            ),
                            xvc_metadata: Some(
                                XvcMetadata {
                                    file_type: File,
                                    size: Some(
                                        8192,
                                    ),
                                    modified: Some(
                                        SystemTime {
                                            tv_sec: 1722024927,
                                            tv_nsec: 262703931,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                },
            },
        },
        poisoned: false,
        ..
    },
    output_diffs: RwLock {
        data: HStore {
            map: {},
        },
        poisoned: false,
        ..
    },
    step_e: XvcEntity(
        2,
        9270302598983016314,
    ),
    step: XvcStep {
        name: "average-age",
    },
    step_command: XvcStepCommand {
        command: "sqlite3 people.db 'SELECT AVG(Age) FROM People;'",
    },
    current_states: RwLock {
        data: HStore {
            map: {
                XvcEntity(
                    2,
                    9270302598983016314,
                ): Running(
                    FromWaitProcess,
                ),
            },
        },
        poisoned: false,
        ..
    },
    step_timeout: 10000s,
    all_steps: HStore {
        map: {
            XvcEntity(
                2,
                9270302598983016314,
            ): XvcStep {
                name: "average-age",
            },
        },
    },
    recorded_dependencies: R1NStore {
        parents: XvcStore {
            map: {
                XvcEntity(
                    2,
                    9270302598983016314,
                ): XvcStep {
                    name: "average-age",
                },
            },
            entity_index: {
                XvcStep {
                    name: "average-age",
                }: [
                    XvcEntity(
                        2,
                        9270302598983016314,
                    ),
                ],
            },
            previous: EventLog(
                [
                    Add {
                        entity: XvcEntity(
                            2,
                            9270302598983016314,
                        ),
                        value: XvcStep {
                            name: "average-age",
                        },
                    },
                    Add {
                        entity: XvcEntity(
                            2,
                            9270302598983016314,
                        ),
                        value: XvcStep {
                            name: "average-age",
                        },
                    },
                ],
            ),
            current: EventLog(
                [],
            ),
        },
        children: XvcStore {
            map: {
                XvcEntity(
                    3,
                    9022053985516504033,
                ): SqliteQueryDigest(
                    SqliteQueryDep {
                        path: XvcPath(
                            "people.db",
                        ),
                        query: "SELECT count(*) FROM People;",
                        query_digest: None,
                        xvc_metadata: None,
                    },
                ),
            },
            entity_index: {
                SqliteQueryDigest(
                    SqliteQueryDep {
                        path: XvcPath(
                            "people.db",
                        ),
                        query: "SELECT count(*) FROM People;",
                        query_digest: None,
                        xvc_metadata: None,
                    },
                ): [
                    XvcEntity(
                        3,
                        9022053985516504033,
                    ),
                ],
            },
            previous: EventLog(
                [
                    Add {
                        entity: XvcEntity(
                            3,
                            9022053985516504033,
                        ),
                        value: SqliteQueryDigest(
                            SqliteQueryDep {
                                path: XvcPath(
                                    "people.db",
                                ),
                                query: "SELECT count(*) FROM People;",
                                query_digest: None,
                                xvc_metadata: None,
                            },
                        ),
                    },
                ],
            ),
            current: EventLog(
                [],
            ),
        },
        child_parents: XvcStore {
            map: {
                XvcEntity(
                    3,
                    9022053985516504033,
                ): ChildEntity(
                    XvcEntity(
                        2,
                        9270302598983016314,
                    ),
                    PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                    PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                ),
            },
            entity_index: {
                ChildEntity(
                    XvcEntity(
                        2,
                        9270302598983016314,
                    ),
                    PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                    PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                ): [
                    XvcEntity(
                        3,
                        9022053985516504033,
                    ),
                ],
            },
            previous: EventLog(
                [
                    Add {
                        entity: XvcEntity(
                            3,
                            9022053985516504033,
                        ),
                        value: ChildEntity(
                            XvcEntity(
                                2,
                                9270302598983016314,
                            ),
                            PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                            PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
                        ),
                    },
                ],
            ),
            current: EventLog(
                [],
            ),
        },
    },
    step_dependencies: {},
    step_outputs: HStore {
        map: {},
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::770] step.name: "average-age"
[TRACE][pipeline/src/pipeline/mod.rs::771] &r_next_state: DoneByRunning(
    FromProcessCompletedSuccessfully,
)
[TRACE][pipeline/src/pipeline/mod.rs::773] &step_state: DoneByRunning(
    FromProcessCompletedSuccessfully,
)
[TRACE][pipeline/src/pipeline/mod.rs::660] &step_state: DoneByRunning(
    FromProcessCompletedSuccessfully,
)
[TRACE][pipeline/src/pipeline/mod.rs::519] "Before state updater": "Before state updater"
[TRACE][pipeline/src/pipeline/mod.rs::529] step_states: RwLock {
    data: HStore {
        map: {
            XvcEntity(
                2,
                9270302598983016314,
            ): DoneByRunning(
                FromProcessCompletedSuccessfully,
            ),
        },
    },
    poisoned: false,
    ..
}
[TRACE][pipeline/src/pipeline/mod.rs::536] done_successfully: Ok(
    true,
)
[TRACE][core/src/util/pmp.rs::185] self.background_thread: Mutex {
    data: JoinHandle { .. },
    poisoned: false,
    ..
}
[TRACE][core/src/util/pmp.rs::190] self.background_thread: Mutex {
    data: JoinHandle { .. },
    poisoned: false,
    ..
}
[TRACE][core/src/util/pmp.rs::99] index: 1
[TRACE][lib/src/cli/mod.rs::376] "Before handle_git_automation": "Before handle_git_automation"
[TRACE][lib/src/git.rs::30] args: [
    "-C",
    "[CWD]",
    "diff",
    "--name-only",
    "--cached",
]
[DEBUG] Using Git: /opt/homebrew/bin/git
[TRACE][lib/src/git.rs::61] git_diff_staged_out: ""
[TRACE][lib/src/git.rs::30] args: [
    "-C",
    "[CWD]",
    "add",
    "--verbose",
    "[CWD]/.xvc",
    "*.gitignore",
    "*.xvcignore",
]
[TRACE][lib/src/git.rs::179] git_add_output: "add '.xvc/store/xvc-dependency-store/1722171156330163.json'
"
[TRACE][lib/src/git.rs::30] args: [
    "-C",
    "[CWD]",
    "commit",
    "-m",
    "Xvc auto-commit after /'/Users/iex/github.com/iesahin/xvc/target/debug/xvc -vvvv pipeline run/'",
]
[DEBUG] Committing .xvc/ to git: [main f7aaa52] Xvc auto-commit after '/Users/iex/github.com/iesahin/xvc/target/debug/xvc -vvvv pipeline run'
 1 file changed, 1 insertion(+)
 create mode 100644 .xvc/store/xvc-dependency-store/1722171156330163.json

[DEBUG] Command completed successfully.

```

But it won't run the step a second time, as the table didn't change.

```console
$ xvc pipeline run

```

Let's add another row to the table:

```console
$ sqlite3 people.db "INSERT INTO People (Name, Sex, Age, Height_in, Weight_lbs) VALUES ('Asude', 'F', 10, 74, 170);"
```

This time, the step will run again as the result from dependency query (`SELECT count(*) FROM People`) changed.

```console
$ xvc pipeline run

```

```note
Xvc opens the database in read-only mode to avoid locking.
```
