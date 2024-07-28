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
$ xvc -vvvv pipeline run
? 101
[DEBUG][logging/src/lib.rs::237] Terminal logger enabled with level: Trace
[TRACE][core/src/types/xvcroot.rs::263] xvc_candidate: "[CWD]/.xvc"
[TRACE][core/src/types/xvcroot.rs::265] parent: "[CWD]"
[DEBUG][config/src/error.rs::72] Config source for level "system" not found at "/Users/iex/Library/Application Support/com.emresult.xvc"
[DEBUG][config/src/error.rs::72] Config source for level "global" not found at "/Users/iex/Library/Application Support/xvc"
[TRACE][ecs/src/ecs/mod.rs::229] dir: "[CWD]/.xvc/ec"
[TRACE][ecs/src/ecs/mod.rs::239] files: [
    "[CWD]/.xvc/ec/1722026597763090",
    "[CWD]/.xvc/ec/1722026597766408",
    "[CWD]/.xvc/ec/1722026597840236",
    "[CWD]/.xvc/ec/1722026598025010",
]
[TRACE][core/src/types/xvcroot.rs::263] xvc_candidate: "[CWD]/.xvc"
[TRACE][core/src/types/xvcroot.rs::265] parent: "[CWD]"
[TRACE][pipeline/src/pipeline/mod.rs::285] pipeline_e: XvcEntity(
    1,
    4733698130976263725,
)
[TRACE][pipeline/src/pipeline/mod.rs::290] pipeline_steps: HStore {
    map: {
        XvcEntity(
            2,
            17595501963111217905,
        ): XvcStep {
            name: "average-age",
        },
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::293] consider_changed: XvcStore {
    map: {
        XvcEntity(
            2,
            17595501963111217905,
        ): ByDependencies,
    },
    entity_index: {
        ByDependencies: [
            XvcEntity(
                2,
                17595501963111217905,
            ),
        ],
    },
    previous: EventLog(
        [
            Add {
                entity: XvcEntity(
                    2,
                    17595501963111217905,
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
    paths: 0x0000600000d70120,
    since_when: 18446744073709551615,
    latency: 0.0,
    flags: 18,
    event_handler: 0x0000600002870010,
    runloop: Some(
        (
            0x00006000036740c0,
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
    paths: 0x0000600000d70120,
    since_when: 18446744073709551615,
    latency: 0.0,
    flags: 18,
    event_handler: 0x0000600002870010,
    runloop: Some(
        (
            0x00006000036740c0,
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
                        "pipeline.default": String(
                            "default",
                        ),
                        "git.command": String(
                            "git",
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "core.verbosity": String(
                            "error",
                        ),
                        "core.guid": String(
                            "2563bb26e79eff77",
                        ),
                        "file.list.show_dot_files": Boolean(
                            false,
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "file.list.no_summary": Boolean(
                            false,
                        ),
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "pipeline.process_pool_size": Integer(
                            4,
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "file.track.force": Boolean(
                            false,
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                        "git.use_git": Boolean(
                            true,
                        ),
                        "file.list.sort": String(
                            "name-desc",
                        ),
                    },
                },
                XvcConfigMap {
                    source: Project,
                    map: {
                        "core.verbosity": String(
                            "error",
                        ),
                        "git.auto_commit": Boolean(
                            true,
                        ),
                        "git.use_git": Boolean(
                            true,
                        ),
                        "file.track.no_parallel": Boolean(
                            false,
                        ),
                        "file.recheck.method": String(
                            "copy",
                        ),
                        "pipeline.default_params_file": String(
                            "params.yaml",
                        ),
                        "git.auto_stage": Boolean(
                            false,
                        ),
                        "file.track.no_commit": Boolean(
                            false,
                        ),
                        "file.list.recursive": Boolean(
                            false,
                        ),
                        "pipeline.default": String(
                            "default",
                        ),
                        "file.list.format": String(
                            "{{aft}}{{rrm}} {{asz}} {{ats}} {{rcd8}} {{acd8}} {{name}}",
                        ),
                        "file.list.show_dot_files": Boolean(
                            false,
                        ),
                        "file.track.text_or_binary": String(
                            "auto",
                        ),
                        "cache.algorithm": String(
                            "blake3",
                        ),
                        "file.list.sort": String(
                            "name-desc",
                        ),
                        "core.guid": String(
                            "fff3d263419bb95f",
                        ),
                        "pipeline.current_pipeline": String(
                            "default",
                        ),
                        "file.carry-in.no_parallel": Boolean(
                            false,
                        ),
                        "file.carry-in.force": Boolean(
                            false,
                        ),
                        "git.command": String(
                            "git",
                        ),
                        "file.list.no_summary": Boolean(
                            false,
                        ),
                        "pipeline.process_pool_size": Integer(
                            4,
                        ),
                        "file.track.force": Boolean(
                            false,
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
                        "core.verbosity": String(
                            "debug",
                        ),
                        "core.quiet": Boolean(
                            false,
                        ),
                    },
                },
            ],
            the_config: {
                "pipeline.current_pipeline": XvcConfigValue {
                    source: Project,
                    value: String(
                        "default",
                    ),
                },
                "file.track.force": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "cache.algorithm": XvcConfigValue {
                    source: Project,
                    value: String(
                        "blake3",
                    ),
                },
                "git.use_git": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        true,
                    ),
                },
                "file.recheck.method": XvcConfigValue {
                    source: Project,
                    value: String(
                        "copy",
                    ),
                },
                "file.track.text_or_binary": XvcConfigValue {
                    source: Project,
                    value: String(
                        "auto",
                    ),
                },
                "core.guid": XvcConfigValue {
                    source: Project,
                    value: String(
                        "fff3d263419bb95f",
                    ),
                },
                "pipeline.default_params_file": XvcConfigValue {
                    source: Project,
                    value: String(
                        "params.yaml",
                    ),
                },
                "file.carry-in.force": XvcConfigValue {
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
                "git.auto_stage": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "git.command": XvcConfigValue {
                    source: Project,
                    value: String(
                        "git",
                    ),
                },
                "file.list.recursive": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
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
                "TRYCMD_TESTS": XvcConfigValue {
                    source: Environment,
                    value: String(
                        "storage,file,pipeline,core",
                    ),
                },
                "git.auto_commit": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        true,
                    ),
                },
                "file.list.sort": XvcConfigValue {
                    source: Project,
                    value: String(
                        "name-desc",
                    ),
                },
                "file.track.no_parallel": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "file.list.show_dot_files": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "pipeline.default": XvcConfigValue {
                    source: Project,
                    value: String(
                        "default",
                    ),
                },
                "core.verbosity": XvcConfigValue {
                    source: CommandLine,
                    value: String(
                        "debug",
                    ),
                },
                "file.list.no_summary": XvcConfigValue {
                    source: Project,
                    value: Boolean(
                        false,
                    ),
                },
                "pipeline.process_pool_size": XvcConfigValue {
                    source: Project,
                    value: Integer(
                        4,
                    ),
                },
                "file.track.no_commit": XvcConfigValue {
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
guid = /"2563bb26e79eff77/"
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
            random: 2745833803509197134,
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
        17595501963111217905,
    ): [],
}
[TRACE][pipeline/src/pipeline/mod.rs::343] &dependency_graph: {
    XvcEntity(
        2,
        17595501963111217905,
    ): [],
}
[INFO][pipeline/src/pipeline/mod.rs::347] Pipeline Graph:
digraph {
    0 [ label = "(2, 17595501963111217905)" ]
}


[TRACE][pipeline/src/pipeline/mod.rs::412] step_states: RwLock {
    data: HStore {
        map: {
            XvcEntity(
                2,
                17595501963111217905,
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
            17595501963111217905,
        ): ScopedJoinHandle { .. },
    },
}
[TRACE][pipeline/src/pipeline/mod.rs::512] (step_e, &jh): (
    XvcEntity(
        2,
        17595501963111217905,
    ),
    ScopedJoinHandle { .. },
)
[TRACE][pipeline/src/pipeline/mod.rs::583] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::623] params.recorded_dependencies: R1NStore {
    parents: XvcStore {
        map: {
            XvcEntity(
                2,
                17595501963111217905,
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
                    17595501963111217905,
                ),
            ],
        },
        previous: EventLog(
            [
                Add {
                    entity: XvcEntity(
                        2,
                        17595501963111217905,
                    ),
                    value: XvcStep {
                        name: "average-age",
                    },
                },
                Add {
                    entity: XvcEntity(
                        2,
                        17595501963111217905,
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
                15601036540397786949,
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
                    15601036540397786949,
                ),
            ],
        },
        previous: EventLog(
            [
                Add {
                    entity: XvcEntity(
                        3,
                        15601036540397786949,
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
                15601036540397786949,
            ): ChildEntity(
                XvcEntity(
                    2,
                    17595501963111217905,
                ),
                PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
            ),
        },
        entity_index: {
            ChildEntity(
                XvcEntity(
                    2,
                    17595501963111217905,
                ),
                PhantomData<xvc_pipeline::pipeline::deps::XvcDependency>,
                PhantomData<xvc_pipeline::pipeline::step::XvcStep>,
            ): [
                XvcEntity(
                    3,
                    15601036540397786949,
                ),
            ],
        },
        previous: EventLog(
            [
                Add {
                    entity: XvcEntity(
                        3,
                        15601036540397786949,
                    ),
                    value: ChildEntity(
                        XvcEntity(
                            2,
                            17595501963111217905,
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
[TRACE][pipeline/src/pipeline/mod.rs::624] step_e: XvcEntity(
    2,
    17595501963111217905,
)
[TRACE][pipeline/src/pipeline/mod.rs::564] dep_neighbors: Neighbors {
    iter: Iter(
        [],
    ),
    ty: PhantomData<petgraph::Directed>,
}
[TRACE][pipeline/src/pipeline/mod.rs::625] dependency_steps(step_e, params.dependency_graph)?: {}
[TRACE][pipeline/src/pipeline/mod.rs::564] dep_neighbors: Neighbors {
    iter: Iter(
        [],
    ),
    ty: PhantomData<petgraph::Directed>,
}
[TRACE][pipeline/src/pipeline/mod.rs::662] &step_state: Begin(
    FromInit,
)
[TRACE][pipeline/src/pipeline/mod.rs::772] step.name: "average-age"
[TRACE][pipeline/src/pipeline/mod.rs::773] &r_next_state: WaitingDependencySteps(
    FromRunConditional,
)
[TRACE][pipeline/src/pipeline/mod.rs::775] &step_state: WaitingDependencySteps(
    FromRunConditional,
)
[TRACE][pipeline/src/pipeline/mod.rs::662] &step_state: WaitingDependencySteps(
    FromRunConditional,
)
[TRACE][pipeline/src/pipeline/mod.rs::772] step.name: "average-age"
[TRACE][pipeline/src/pipeline/mod.rs::773] &r_next_state: CheckingOutputs(
    FromDependencyStepsFinishedSuccessfully,
)
[TRACE][pipeline/src/pipeline/mod.rs::775] &step_state: CheckingOutputs(
    FromDependencyStepsFinishedSuccessfully,
)
[TRACE][pipeline/src/pipeline/mod.rs::662] &step_state: CheckingOutputs(
    FromDependencyStepsFinishedSuccessfully,
)
[TRACE][pipeline/src/pipeline/mod.rs::772] step.name: "average-age"
[TRACE][pipeline/src/pipeline/mod.rs::773] &r_next_state: CheckingSuperficialDiffs(
    FromCheckedOutputs,
)
[TRACE][pipeline/src/pipeline/mod.rs::775] &step_state: CheckingSuperficialDiffs(
    FromCheckedOutputs,
)
[TRACE][pipeline/src/pipeline/mod.rs::662] &step_state: CheckingSuperficialDiffs(
    FromCheckedOutputs,
)
[TRACE][pipeline/src/pipeline/mod.rs::1010] parent_entity: XvcEntity(
    2,
    17595501963111217905,
)
[TRACE][pipeline/src/pipeline/mod.rs::583] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::1013] deps: HStore {
    map: {
        XvcEntity(
            3,
            15601036540397786949,
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
[TRACE][pipeline/src/pipeline/mod.rs::583] select: Select { .. }
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
[TRACE][pipeline/src/pipeline/mod.rs::583] select: Select { .. }
[TRACE][pipeline/src/pipeline/mod.rs::583] select: Select { .. }
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
[TRACE][pipeline/src/pipeline/mod.rs::1030] step_dependency_diffs: HStore {
    map: {
        XvcEntity(
            3,
            15601036540397786949,
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
[TRACE][pipeline/src/pipeline/mod.rs::1036] diff: RecordMissing {
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
[INFO] No dependency steps for step average-age
[TRACE][pipeline/src/pipeline/mod.rs::1037] diff.changed(): true
[TRACE][pipeline/src/pipeline/mod.rs::1042] changed: true
[TRACE][pipeline/src/pipeline/mod.rs::772] step.name: "average-age"
[TRACE][pipeline/src/pipeline/mod.rs::773] &r_next_state: CheckingThoroughDiffs(
    FromSuperficialDiffsChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::775] &step_state: CheckingThoroughDiffs(
    FromSuperficialDiffsChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::662] &step_state: CheckingThoroughDiffs(
    FromSuperficialDiffsChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::1067] deps: HStore {
    map: {
        XvcEntity(
            3,
            15601036540397786949,
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
[TRACE][pipeline/src/pipeline/mod.rs::583] select: Select { .. }
thread '<unnamed>' panicked at pipeline/src/pipeline/mod.rs:1081:32:
SqliteError { source: FromSqlConversionFailure(18446744073709551615, Null, InvalidType) }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread '<unnamed>' panicked at lib/src/cli/mod.rs:288:52:
[PANIC] SqliteError { source: FromSqlConversionFailure(18446744073709551615, Null, InvalidType) }, [pipeline/src/pipeline/mod.rs::1081]
[TRACE][pipeline/src/pipeline/mod.rs::519] "Before state updater": "Before state updater"
[TRACE][pipeline/src/pipeline/mod.rs::593] s: CheckingThoroughDiffs(
    FromSuperficialDiffsChanged,
)
[TRACE][pipeline/src/pipeline/mod.rs::532] step_states: RwLock {
    data: HStore {
        map: {
            XvcEntity(
                2,
                17595501963111217905,
            ): CheckingThoroughDiffs(
                FromSuperficialDiffsChanged,
            ),
        },
    },
    poisoned: false,
    ..
}
[TRACE][pipeline/src/pipeline/mod.rs::536] done_successfully: Ok(
    false,
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
[TRACE][lib/src/cli/mod.rs::376] "Before handle_git_automation": "Before handle_git_automation"
[TRACE][core/src/util/pmp.rs::99] index: 1
thread '<unnamed>' panicked at lib/src/git.rs:155:5:
called `Result::unwrap()` on an `Err` value: "SendError(..)"
thread 'main' panicked at lib/src/cli/mod.rs:402:52:
called `Result::unwrap()` on an `Err` value: Any { .. }

```

But it won't run the step a second time, as the table didn't change.

```console
$ xvc pipeline run
? interrupted
thread '<unnamed>' panicked at pipeline/src/pipeline/mod.rs:1081:32:
SqliteError { source: FromSqlConversionFailure(18446744073709551615, Null, InvalidType) }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread '<unnamed>' panicked at lib/src/cli/mod.rs:251:52:
[PANIC] SqliteError { source: FromSqlConversionFailure(18446744073709551615, Null, InvalidType) }, [pipeline/src/pipeline/mod.rs::1081]
thread '<unnamed>' panicked at pipeline/src/pipeline/mod.rs:514:17:
called `Result::unwrap()` on an `Err` value: "SendError(..)"

```

Let's add another row to the table:

```console
$ sqlite3 people.db "INSERT INTO People (Name, Sex, Age, Height_in, Weight_lbs) VALUES ('Asude', 'F', 10, 74, 170);"
```

This time, the step will run again as the result from dependency query (`SELECT count(*) FROM People`) changed.

```console
$ xvc pipeline run
? interrupted
thread '<unnamed>' panicked at pipeline/src/pipeline/mod.rs:1081:32:
SqliteError { source: FromSqlConversionFailure(18446744073709551615, Null, InvalidType) }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread '<unnamed>' panicked at lib/src/cli/mod.rs:251:52:
[PANIC] SqliteError { source: FromSqlConversionFailure(18446744073709551615, Null, InvalidType) }, [pipeline/src/pipeline/mod.rs::1081]
thread '<unnamed>' panicked at pipeline/src/pipeline/mod.rs:514:17:
called `Result::unwrap()` on an `Err` value: "SendError(..)"

```

```note
Xvc opens the database in read-only mode to avoid locking.
```
