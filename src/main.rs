

mod cpu {

    const NUM_GPR: usize = 32;

    struct Fpcsr(u32);

    impl Fpcsr {

        ///
        fn as_u32(&self) -> u32 {
            return self.0;
        }

    }

    struct Cpu {
        gpr: [u32; NUM_GPR],
        fpr: [f64; NUM_GPR],

        fpcsr: Fpcsr

    }

}



fn main() {
    println!("Hello, world!");
}
