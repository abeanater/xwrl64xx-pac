#[doc = "Register `FEC_CM3_CTI_INEN7` reader"]
pub type R = crate::R<FecCm3CtiInen7Spec>;
#[doc = "Register `FEC_CM3_CTI_INEN7` writer"]
pub type W = crate::W<FecCm3CtiInen7Spec>;
#[doc = "Field `FEC_CM3_CTI_INEN7` reader - "]
pub type FecCm3CtiInen7R = crate::FieldReader<u32>;
#[doc = "Field `FEC_CM3_CTI_INEN7` writer - "]
pub type FecCm3CtiInen7W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn fec_cm3_cti_inen7(&self) -> FecCm3CtiInen7R {
        FecCm3CtiInen7R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn fec_cm3_cti_inen7(&mut self) -> FecCm3CtiInen7W<FecCm3CtiInen7Spec> {
        FecCm3CtiInen7W::new(self, 0)
    }
}
#[doc = "FEC_CM3_CTI_INEN7\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_inen7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_inen7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FecCm3CtiInen7Spec;
impl crate::RegisterSpec for FecCm3CtiInen7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fec_cm3_cti_inen7::R`](R) reader structure"]
impl crate::Readable for FecCm3CtiInen7Spec {}
#[doc = "`write(|w| ..)` method takes [`fec_cm3_cti_inen7::W`](W) writer structure"]
impl crate::Writable for FecCm3CtiInen7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FEC_CM3_CTI_INEN7 to value 0"]
impl crate::Resettable for FecCm3CtiInen7Spec {
    const RESET_VALUE: u32 = 0;
}
