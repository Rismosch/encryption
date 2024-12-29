// https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.197-upd1.pdf
pub fn run() {
    let message = "Today when I walked into my economics class I saw something I dread every time I close my eyes. Someone had brought their new gaming laptop to class. The Forklift he used to bring it was still running idle at the back. I started sweating as I sat down and gazed over at the 700lb beast that was his laptop. He had already reinforced his desk with steel support beams and was in the process of finding an outlet for a power cable thicker than Amy Schumer's thigh. I start shaking. I keep telling myself I'm going to be alright and that there's nothing to worry about. He somehow finds a fucking outlet. Tears are running down my cheeks as I send my last texts to my family saying I love them. The teacher starts the lecture, and the student turns his laptop on. The colored lights on his RGB Backlit keyboard flare to life like a nuclear flash, and a deep humming fills my ears and shakes my very soul. The entire city power grid goes dark. The classroom begins to shake as the massive fans begin to spin. In mere seconds my world has gone from vibrant life, to a dark, earth shattering void where my body is getting torn apart by the 150mph gale force winds and the 500 decibel groan of the cooling fans. As my body finally surrenders, I weep, as my school and my city go under. I fucking hate gaming laptops.";

    let input = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    let s = array_to_state(input);
    let output = state_to_array(s);

    println!("input:\n{:#?}", input);
    println!("s:\n{:#?}", s);
    println!("output:\n{:#?}", output);
    println!("column:\n{:#?}", state_column(s, 0));
    println!("column:\n{:#?}", state_column(s, 1));
    println!("column:\n{:#?}", state_column(s, 2));
    println!("column:\n{:#?}", state_column(s, 3));
}

type Array = [u8; 16];
type State = Block;
type Word = [u8; 4];
type Block = [Word; 4];

fn array_to_state(input: Array) -> State {
    let mut s = State::default();
    s[0][0] = input[0+4*0];
    s[0][1] = input[0+4*1];
    s[0][2] = input[0+4*2];
    s[0][3] = input[0+4*3];
    s[1][0] = input[1+4*0];
    s[1][1] = input[1+4*1];
    s[1][2] = input[1+4*2];
    s[1][3] = input[1+4*3];
    s[2][0] = input[2+4*0];
    s[2][1] = input[2+4*1];
    s[2][2] = input[2+4*2];
    s[2][3] = input[2+4*3];
    s[3][0] = input[3+4*0];
    s[3][1] = input[3+4*1];
    s[3][2] = input[3+4*2];
    s[3][3] = input[3+4*3];
    s
}

fn state_to_array(s: State) -> Array {
    let mut output = Array::default();
    output[0+4*0] = s[0][0];
    output[0+4*1] = s[0][1];
    output[0+4*2] = s[0][2];
    output[0+4*3] = s[0][3];
    output[1+4*0] = s[1][0];
    output[1+4*1] = s[1][1];
    output[1+4*2] = s[1][2];
    output[1+4*3] = s[1][3];
    output[2+4*0] = s[2][0];
    output[2+4*1] = s[2][1];
    output[2+4*2] = s[2][2];
    output[2+4*3] = s[2][3];
    output[3+4*0] = s[3][0];
    output[3+4*1] = s[3][1];
    output[3+4*2] = s[3][2];
    output[3+4*3] = s[3][3];
    output
}

fn state_column(s: State, c: usize) -> Word {
    let mut word = Word::default();
    word[0] = s[0][c];
    word[1] = s[1][c];
    word[2] = s[2][c];
    word[3] = s[3][c];
    word
}

fn gf_2_8_add(lhs: u8, rhs: u8) -> u8 {
    lhs ^ rhs
}

fn gf_2_8_mul(lhs: u8, rhs: u8) -> u8 {

}

fn add_round_key() {}
fn aes_128() {}
fn aes_192() {}
fn aes_256() {}
fn cipher() {}
fn eq_inv_cipher() {}
fn inv_cipher() {}
fn inv_mix_columns() {}
fn inv_s_box() {}
fn inv_shift_rows() {}
fn inv_sub_bytes() {}
fn key_expansion() {}
fn key_expansion_eic() {}
fn mix_columns() {}
fn rot_word() {}
fn s_box() {}
fn shift_rows() {}
fn sub_bytes() {}
fn sub_word() {}
fn x_times() {}

