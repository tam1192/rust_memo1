// テクスチャー定義
#[derive(Clone, Copy, Debug)]
enum Texture {
    Block,
    Player,
    Air,
}

// テスクチャーのディスプレイ実装
impl fmt::Display for Texture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = match self {
            Texture::Block => "■",
            Texture::Player => "A",
            Texture::Air => " ",
        };
        write!(f, "{}", output)
    }
}
