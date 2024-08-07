#[doc = "Register `RESERVED0` reader"]
pub type R = crate::R<Reserved0Spec>;
#[doc = "Register `RESERVED0` writer"]
pub type W = crate::W<Reserved0Spec>;
impl W {}
#[doc = "RESERVED0\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reserved0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reserved0Spec;
impl crate::RegisterSpec for Reserved0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reserved0::R`](R) reader structure"]
impl crate::Readable for Reserved0Spec {}
#[doc = "`write(|w| ..)` method takes [`reserved0::W`](W) writer structure"]
impl crate::Writable for Reserved0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RESERVED0 to value 0"]
impl crate::Resettable for Reserved0Spec {
    const RESET_VALUE: u32 = 0;
}
