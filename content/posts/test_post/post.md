# Post number 1
## The best of them all

This is a test article, it is not intended to ship in production. It is written in markdown and, ideally, it should be parsed to html.

```rust
const TEST: &'static str = "test string";

// this is a comment
pub fn look(value: bool){
    if value == true{
        let float = 14.5f32;

        i_can_display_some_code();
    }
}
```

```javascript
let variable = true

function test(test1, test2){
    console.log("hiiiii")
}
```

```csharp
namespace Test.Program
{
    class Main
    {
        private readonly ILogger _logger;

        public Main(ILogger<Main> logger)
        {
            _logger = logger;
        }

        public static void Main(string[] args)
        {
            if(_logger.Status == Status.Active)
            {
                Console.WriteLine("test");
                Console.WriteLite($"test {14.ToString()}");
            }
        }
    }
}
```

```json
{
    "test": "json",
    "list": [
        "item1",
        2,
        2.5,
        "item3",
        "https://github.com"
    ]
}
```

```css
:root{
    --black: #181819;
    --bg-dim: #222327;
    --bg0: #2c2e34;
    --bg1: #33353f;
    --bg2: #363944;
    --bg3: #3b3e48;
    --bg4: #414550;
    --bg-red: #ff6077;
    --diff-red: #55393d;
    --bg-green: #a7df78;
    --diff-green: #394634;
    --bg-blue: #85d3f2;
    --diff-blue: #354157;
    --diff-yellow: #4e432f;
    --fg: #e2e2e3;
    --red: #fc5d7c;
    --orange: #f39660;
    --yellow: #e7c664;
    --green: #9ed072;
    --blue: #76cce0;
    --purple: #b39df3;
    --grey: #7f8490;
    --grey-dim: #595f6f;
}

.highlighted-code{
    background: var(--bg0);
    color: var(--fg);
    font-style: normal;
}

.highlighted-code .hh0{ /* annotation */
    color: var(--blue);
    font-style: italic;
}
.highlighted-code .hh1{ /* attribute */
    color: var(--blue);
    font-style: italic;
}
.highlighted-code .hh2{ /* boolean */
    color: var(--purple);
    font-style: normal;
}
```
