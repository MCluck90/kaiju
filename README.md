# kaiju - TUI Project Manager

Quickly open your projects from anywhere.

## Installation

```sh
git clone https://github.com/MCluck90/kaiju
cd kaiju
cargo build --release
ln -s $PWD/target/release/kaiju /somewhere/in/PATH/kj
```

## Usage

```sh
kj
```

## Configuration

The config file is expected to be in `~/.config/kaiju.json`.

Schema presented as a TypeScript type:

```ts
interface KaijuConfig {
  projects: Project[];
}

interface Project {
  name: string;
  path: string;
}
```

### Example

```json
{
  "projects": [
    {
      "name": "kaiju",
      "path": "~/code/kaiju"
    }
  ]
}
```
