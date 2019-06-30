use bright::*;

macro_rules! output {
    ( $( $x:expr ),* ) => {
        {
            $(
                print!("{} ", $x);
            )*
            print!("\n");
        }
    };
}

fn main() {
    output![
        " bold ".bold(),
        "  dim ".dim(),
        " italic ".italic(),
        "underline ".underline(),
        " fast_blink ".fast_blink(),
        " invert ".invert()
    ];

    output![
        " black ".black(),
        " red ".red(),
        " green ".green(),
        " yellow ".yellow(),
        " blue ".blue(),
        " magenta ".magenta(),
        " cyan ".cyan(),
        " white ".white()
    ];

    output![
        " black ".bg_black(),
        " red ".bg_red(),
        " green ".bg_green(),
        " yellow ".bg_yellow(),
        " blue ".bg_blue(),
        " magenta ".bg_magenta(),
        " cyan ".bg_cyan(),
        " white ".bg_white()
    ];

    output![" RGB ".rgb(40, 44, 52).bg_rgb(3, 169, 244)];
}
