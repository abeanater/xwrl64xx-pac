#[doc = "Register `APP_CM4_CTI_APPPULSE` reader"]
pub type R = crate::R<AppCm4CtiApppulseSpec>;
#[doc = "Register `APP_CM4_CTI_APPPULSE` writer"]
pub type W = crate::W<AppCm4CtiApppulseSpec>;
#[doc = "Field `APP_CM4_CTI_APPPULSE` reader - "]
pub type AppCm4CtiApppulseR = crate::FieldReader<u32>;
#[doc = "Field `APP_CM4_CTI_APPPULSE` writer - "]
pub type AppCm4CtiApppulseW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn app_cm4_cti_apppulse(&self) -> AppCm4CtiApppulseR {
        AppCm4CtiApppulseR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn app_cm4_cti_apppulse(&mut self) -> AppCm4CtiApppulseW<AppCm4CtiApppulseSpec> {
        AppCm4CtiApppulseW::new(self, 0)
    }
}
#[doc = "APP_CM4_CTI_APPPULSE\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cm4_cti_apppulse::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cm4_cti_apppulse::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppCm4CtiApppulseSpec;
impl crate::RegisterSpec for AppCm4CtiApppulseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`app_cm4_cti_apppulse::R`](R) reader structure"]
impl crate::Readable for AppCm4CtiApppulseSpec {}
#[doc = "`write(|w| ..)` method takes [`app_cm4_cti_apppulse::W`](W) writer structure"]
impl crate::Writable for AppCm4CtiApppulseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APP_CM4_CTI_APPPULSE to value 0"]
impl crate::Resettable for AppCm4CtiApppulseSpec {
    const RESET_VALUE: u32 = 0;
}
