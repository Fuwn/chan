# chan
A simple "chan"-like board written in Rust.

[![GitHub Issues](https://img.shields.io/github/issues-raw/fuwn/chan.svg)](https://github.com/fuwn/chan/issues)
[![License](https://img.shields.io/github/license/fuwn/chan.svg)](LICENSE)
![GitHub Stars](https://img.shields.io/github/stars/fuwn/chan?style=social)

```
⣿⣿⣷⡁⢆⠈⠕⢕⢂⢕⢂⢕⢂⢔⢂⢕⢄⠂⣂⠂⠆⢂⢕⢂⢕⢂⢕⢂⢕⢂
⣿⣿⣿⡷⠊⡢⡹⣦⡑⢂⢕⢂⢕⢂⢕⢂⠕⠔⠌⠝⠛⠶⠶⢶⣦⣄⢂⢕⢂⢕
⣿⣿⠏⣠⣾⣦⡐⢌⢿⣷⣦⣅⡑⠕⠡⠐⢿⠿⣛⠟⠛⠛⠛⠛⠡⢷⡈⢂⢕⢂
⠟⣡⣾⣿⣿⣿⣿⣦⣑⠝⢿⣿⣿⣿⣿⣿⡵⢁⣤⣶⣶⣿⢿⢿⢿⡟⢻⣤⢑⢂
⣾⣿⣿⡿⢟⣛⣻⣿⣿⣿⣦⣬⣙⣻⣿⣿⣷⣿⣿⢟⢝⢕⢕⢕⢕⢽⣿⣿⣷⣔
⣿⣿⠵⠚⠉⢀⣀⣀⣈⣿⣿⣿⣿⣿⣿⣿⣿⣿⣗⢕⢕⢕⢕⢕⢕⣽⣿⣿⣿⣿
⢷⣂⣠⣴⣾⡿⡿⡻⡻⣿⣿⣴⣿⣿⣿⣿⣿⣿⣷⣵⣵⣵⣷⣿⣿⣿⣿⣿⣿⡿
⢌⠻⣿⡿⡫⡪⡪⡪⡪⣺⣿⣿⣿⣿⣿⠿⠿⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠃
⠣⡁⠹⡪⡪⡪⡪⣪⣾⣿⣿⣿⣿⠋⠐⢉⢍⢄⢌⠻⣿⣿⣿⣿⣿⣿⣿⣿⠏⠈
⡣⡘⢄⠙⣾⣾⣾⣿⣿⣿⣿⣿⣿⡀⢐⢕⢕⢕⢕⢕⡘⣿⣿⣿⣿⣿⣿⠏⠠⠈
⠌⢊⢂⢣⠹⣿⣿⣿⣿⣿⣿⣿⣿⣧⢐⢕⢕⢕⢕⢕⢅⣿⣿⣿⣿⡿⢋⢜⠠⠈
⠄⠁⠕⢝⡢⠈⠻⣿⣿⣿⣿⣿⣿⣿⣷⣕⣑⣑⣑⣵⣿⣿⣿⡿⢋⢔⢕⣿⠠⠈
⠨⡂⡀⢑⢕⡅⠂⠄⠉⠛⠻⠿⢿⣿⣿⣿⣿⣿⣿⣿⣿⡿⢋⢔⢕⢕⣿⣿⠠⠈
⠄⠪⣂⠁⢕⠆⠄⠂⠄⠁⡀⠂⡀⠄⢈⠉⢍⢛⢛⢛⢋⢔⢕⢕⢕⣽⣿⣿⠠⠈
```

## Inspirational
Originally, this project was supposed to be a public [OpenPGP](https://www.openpgp.org/) key server because I was frustrated with the current selection of key servers, but I randomly had the idea to make a "chan"-like forum and I had this sitting around so I made this!

## Limitations
At the moment, the board is missing automatic post pruning and moderation features, but I have plans to finish adding them in the near future. As well, at the moment, there has to be a 'empty' thread entry in the 'threads' table within the database, for now. This is becaus of the way that the thread making feature grabs valid board data. I'll probably fix this sometime soon also.

Ignore my messy Handlebars templates, I didn't really have a purpose to make them fancy or whatever because of the simplicity of the project.

## Usage
1. Install [Rust](https://www.rust-lang.org/).
2. Clone the repository.
3. `$ cargo run`.

### Adding boards
To add boards, open the example database in a SQLite viewer add a board entry to the 'boards' table (examples provided in database).

Pre-built binaries *might* be available in the future if there is enough demand for them.

## Contribution
Feel free to make pull requests or open issues if you think a feature should be added or you find a bug, or just anything --- within reason.

### License
[GNU Affero General Public License v3.0](https://github.com/fuwn/chan/blob/main/LICENSE.md)
