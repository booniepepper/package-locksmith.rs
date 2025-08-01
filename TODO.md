# To Do

> Note: This is not meant to be comprehensive or precise. It should cover the
  primary or most general/common relationships. Some terms are used loosely,
  like calling sum.golang.org a "repository" is really stretching the
  definition.

| Lock(ish) file    | Format | Language(s)            | Package Manager(s)     | Repositor(y|ies)    |
|-------------------+--------+------------------------+------------------------+---------------------|
| *.opam.lockfile   | Unique | OCaml                  | opam + opam-lock       | opam.ocaml.org      |
| build.zig.zon     | ZON    | Zig, C                 | zig                    | n/a                 |
| bun.lock          | JSON   | JavaScript, TypeScript | bun                    | npmjs.com           |
| Cargo.lock        | TOML   | Rust                   | cargo                  | crates.io           |
| conan.lock        | JSON   | C++, C                 | conan                  | conan.io            |
| composer.lock     | JSON   | PHP                    | composer               | packagist.org       |
| conda-lock.yml    | YAML   | Python, R              | conda + conda-lock     | anaconda.org        |
| deno.lock         | JSON   | JavaScript, TypeScript | deno                   | npmjs.com           |
| Gemfile.lock      | Unique | Ruby                   | bundler                | rubygems.org        |
| go.sum            | Unique | Go                     | go                     | sum.golang.org      |
| gradle.lockfile   | Unique | Java, Kotlin, Groovy   | gradle                 | maven.org           |
| lake-manifest.json | JSON  | Lean                   | lake               | reservoir.lean-lang.org |
| lockfile.jdn      | JDN    | Janet                  | jpm             | github.com/janet-lang/pkgs |
| luarocks.lock     | Lua    | Lua                    | luarocks               | luarocks.org        |
| mix.lock          | Unique | Elixir, Erlang, Gleam  | mix                    | hex.pm              |
| nimble.lock       | JSON   | Nim                    | nimble                 | nimble.directory    |
| package-lock.json | JSON   | JavaScript, TypeScript | npm                    | npmjs.com           |
| packages.lock.json | JSON  | C#, F#, VB.NET         | nuget                  | nuget.org           |
| Pipfile.lock      | Unique | Python                 | pipenv                 | pypi.org            |
| pnpm-lock.yaml    | YAML   | JavaScript, TypeScript | pnpm                   | npmjs.com           |
| Podfile.lock      | Unique | Swift, Objective C     | cocoapods              | cocoapods.org       |
| poetry.lock       | TOML   | Python                 | poetry                 | pypi.org            |
| pom.lockfile.xml  | XML    | Java                   | maven + maven-lockfile | maven.org           |
| pubspeck.lock     | YAML   | Dart, Flutter          | pub                    | pub.dev             |
| pylock.toml       | TOML   | Python                 | (new, many converging) | pypi.org            |
| requirements.txt* | Unique | Python                 | setuptools, others     | pypi.org            |
| stack.yaml.lock   | YAML   | Haskell                | stack                  | hackage.haskell.org |
| uv.lock           | TOML   | Python                 | uv                     | pypi.org            |
| yarn.lock         | Unique | JavaScript, TypeScript | yarn                   | npmjs.com           |

> *: `requirements.txt` _can function_ as a lock file, but is not required to
  contain all direct/transitive dependencies, a package's hash, or even a
  package's version.

---

Some ecosystem relationships without a lock file (as far as I know)

* Clojure, deps.edn/tools.build, clojars.org
* Groovy, Grape, maven.org
* Julia, Pkg, juliahub.com
* Perl, cpan, cpan.org
* Racket, raco, pkgs.racket-lang.org
* Raku, zef, raku.land
* TeX, (options exist), ctan.org
