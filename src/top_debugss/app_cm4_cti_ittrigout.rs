#[doc = "Register `APP_CM4_CTI_ITTRIGOUT` reader"]
pub type R = crate::R<AppCm4CtiIttrigoutSpec>;
#[doc = "Register `APP_CM4_CTI_ITTRIGOUT` writer"]
pub type W = crate::W<AppCm4CtiIttrigoutSpec>;
#[doc = "Field `APP_CM4_CTI_ITTRIGOUT` reader - "]
pub type AppCm4CtiIttrigoutR = crate::FieldReader<u32>;
#[doc = "Field `APP_CM4_CTI_ITTRIGOUT` writer - "]
pub type AppCm4CtiIttrigoutW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn app_cm4_cti_ittrigout(&self) -> AppCm4CtiIttrigoutR {
        AppCm4CtiIttrigoutR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn app_cm4_cti_ittrigout(&mut self) -> AppCm4CtiIttrigoutW<AppCm4CtiIttrigoutSpec> {
        AppCm4CtiIttrigoutW::new(self, 0)
    }
}
#[doc = "APP_CM4_CTI_ITTRIGOUT\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cm4_cti_ittrigout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cm4_cti_ittrigout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppCm4CtiIttrigoutSpec;
impl crate::RegisterSpec for AppCm4CtiIttrigoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`app_cm4_cti_ittrigout::R`](R) reader structure"]
impl crate::Readable for AppCm4CtiIttrigoutSpec {}
#[doc = "`write(|w| ..)` method takes [`app_cm4_cti_ittrigout::W`](W) writer structure"]
impl crate::Writable for AppCm4CtiIttrigoutSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APP_CM4_CTI_ITTRIGOUT to value 0"]
impl crate::Resettable for AppCm4CtiIttrigoutSpec {
    const RESET_VALUE: u32 = 0;
}
