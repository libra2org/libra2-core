module poc::native_print {
   use libra2_std::debug;

   public entry fun main(_owner: &signer) {
      let x = 42u64;
      debug::print(&x);
   }

  #[test(owner=@0x123)]
  fun a(owner:&signer){
     main(owner);
   }
}
