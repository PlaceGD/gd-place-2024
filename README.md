# GD Place Wallpaper Engine Clock

This is a version of the GD Place 2024 countdown designed to be used as a clock within Wallpaper Engine. You can find the link to the wallpaper [here]().

If you encounter any issues, please report them. We can't guarantee they will be fixed, as this was mostly just a silly idea, but please report them anyway. If you do make a report, enabled logging inside of the config (at `CONFIG PATH`) and include the output of the log file, available in the same directory as the log file.

Although it should now be fixed (🤞), I have previously encountered an issue where the wallpaper will take a few tries for it to set (the window will blink open and then close a number of times before if chages the wallpaper). I am not sure what causes this but, if you wait long enough, it will eventually set as the wallpaper.

Unfortunately, this is only available on Windows. Although Wallpaper Engine is Windows only, I did my best to keep it cross platform but the windowing library used (winit) was not compatible with Wallpaper Engine, so it had to be switched out to something Windows-only.

## Config Options

### General

#### `zoom`

Type: `float`

Default: `-6.0`

Required: `true`

Description: Controls the zoom level of the view. Negative values zoom out, positive values zoom in.

#### `fps-cap`

Type: `int`

Default: `60`

Required: `true`

Description: Limits the maximum frames per second to control performance and CPU usage.

#### `rng-seed`

Type: `int`

Default: N/A

Required: `false`

Description: Seed value for random number generation. Use this to reproduce the same random output.

#### `logging`

Type: `boolean`

Default: `true`

Required: `true`

Description: Enables debug logging. Useful if there is an issue with the wallpaper.

### Clock

#### `position`

Type: `center` | `top-left` | `top-right` | `bottom-left` | `bottom-right`

Default: `center`

Required: `true`

Description: Controls where the clock is positioned on the screen.

#### `padding`

Type: `{ top: int, left: int, botton: int, right: int }`

Default: N/A

Required: `false`

Description: Adds extra spacing between the clock and screen edges, in [units](#units).

### Grid

#### `opacity`

Type: `float`

Default: `100.0`

Required: `true`

Description: Opacity of the background grid (0.0 = fully invisible, 100.0 = fully visible).

### Sets

#### `colon-sets`

Type: `int[]`

Default: `[0..5)`

Required: `true`

Description: List of colon types (from 0 to 4) to randomly pick from.

#### `digit-sets`

Type: `int[]`

Default: `[0..85)`

Required: `true`

Description: List of digit styles (from 0 to 84) to randomly pick from. See [colon credits](#colon-credits).

#### `sets`

Type: `{ hours: int, minutes: int, seconds: int, colonh: int, colonm: int }`

Default: N/A

Required: `false`

Description: Overrides `digit-sets` and `colon-sets`. Sets specific styles for each digit and colon. See [digit credits](#digit-credits).

#### `show-colons`

Type: `boolean`

Default: `true`

Required: `true`

Description: Determines whether to show the colons separating the clock digits.

#### `digit-change-frequency`

Type: `int`

Default: `1200`

Required: `true`

Description: Time in seconds between each random regeneration of digits and colons.

### Background

#### `color`

Type: `{ r: int, g: int, b: int, a: int }`

Default: `{ r: 2, g: 12, b: 24, a: 255 }`

Required: `true`

Description: Background color behind the image. Used when the image is hidden or doesn’t fill the screen. Setting all values to 0 makes the window fully transparent (pre-multiplied alpha).

#### `image-tint`

Type: `{ r: int, g: int, b: int, a: int }`

Default: `{ r: 4, g: 24, b: 46, a: 100 }`

Required: `true`

Description: Color used to tint the background image. It is multiplied with the image colors.

#### `fit`

Type: `fill` | `cover` | `contain` | `tile` | `hidden` | `none`

Default: `tile`

Required: `true`

Description: Specifies how the background image fits the screen (e.g., stretched, tiled, hidden, etc.). Based on the [CSS `object-fit` values](https://developer.mozilla.org/en-US/docs/Web/CSS/object-fit#values).

#### `image`

Type: `string`

Default: `./background`

Required: `true`

Description: File path to the background image used behind the clock.

## Units

30 Units is 1 grid space.

## Colon Credits

Made by [GD Colon](https://github.com/GDColon).

## Digit Credits

| Index | Creator Name         |
|-------|----------------------|
| 0     | Spu7Nix              |
| 1     | Viprin               |
| 2     | Cometface            |
| 3     | Flow                 |
| 4     | G4lvatron            |
| 5     | Fungifity            |
| 6     | Thomartin            |
| 7     | DreamingInsanity     |
| 8     | Echonox              |
| 9     | TamaN                |
| 10    | SrGuillester         |
| 11    | Bianox               |
| 12    | SirHadoken           |
| 13    | Jenkins              |
| 14    | KINGTONY             |
| 15    | Dominus              |
| 16    | JonathanGD           |
| 17    | Exyl                 |
| 18    | Jeyzor               |
| 19    | Vermillion           |
| 20    | MelX0exe             |
| 21    | MelX0exe             |
| 22    | EricVanWilderman     |
| 23    | Serponge             |
| 24    | bli                  |
| 25    | Grax                 |
| 26    | KrmaL                |
| 27    | DavJT                |
| 28    | AudieoVisual         |
| 29    | Doggie               |
| 30    | Pocke                |
| 31    | Subwoofer            |
| 32    | para                 |
| 33    | WerewolfGD           |
| 34    | Kips                 |
| 35    | Motleyorc            |
| 36    | Nasgubb              |
| 37    | Tchotchke            |
| 38    | DreamingInsanity     |
| 39    | YunHaSeu             |
| 40    | Rafer                |
| 41    | ILRELL               |
| 42    | Culuc                |
| 43    | BoldStep             |
| 44    | Evasium              |
| 45    | Dorami               |
| 46    | npesta               |
| 47    | Partition            |
| 48    | vrymer               |
| 49    | meeloz               |
| 50    | Flow                 |
| 51    | Glittershroom        |
| 52    | xloco                |
| 53    | Technical            |
| 54    | connot               |
| 55    | Rustam               |
| 56    | RobTopGames          |
| 57    | DesTicY              |
| 58    | xenoteric            |
| 59    | logiking             |
| 60    | AeonAir              |
| 61    | Breadking            |
| 62    | Neigefeu             |
| 63    | Juniper              |
| 64    | Wulzy                |
| 65    | Platnuu              |
| 66    | RadiationV2          |
| 67    | Smiffy777            |
| 68    | Knots                |
| 69    | Terron               |
| 70    | Aquatias             |
| 71    | Thedevon             |
| 72    | Digitalight          |
| 73    | chunlv1              |
| 74    | Stormfly             |
| 75    | verticallity         |
| 76    | Goose                |
| 77    | Voxicat              |
| 78    | Knobbelboy           |
| 79    | ImMaxX1              |
| 80    | DangerKat            |
| 81    | Perox8               |
| 82    | ImMaxX               |
| 83    | Zoink                |
| 84    | Sakura               |
