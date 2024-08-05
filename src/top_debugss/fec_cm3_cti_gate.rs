#[doc = "Register `FEC_CM3_CTI_GATE` reader"]
pub type R = crate::R<FecCm3CtiGateSpec>;
#[doc = "Register `FEC_CM3_CTI_GATE` writer"]
pub type W = crate::W<FecCm3CtiGateSpec>;
#[doc = "Field `FEC_CM3_CTI_GATE` reader - "]
pub type FecCm3CtiGateR = crate::FieldReader<u32>;
#[doc = "Field `FEC_CM3_CTI_GATE` writer - "]
pub type FecCm3CtiGateW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn fec_cm3_cti_gate(&self) -> FecCm3CtiGateR {
        FecCm3CtiGateR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn fec_cm3_cti_gate(&mut self) -> FecCm3CtiGateW<FecCm3CtiGateSpec> {
        FecCm3CtiGateW::new(self, 0)
    }
}
#[doc = "FEC_CM3_CTI_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`fec_cm3_cti_gate::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fec_cm3_cti_gate::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FecCm3CtiGateSpec;
impl crate::RegisterSpec for FecCm3CtiGateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fec_cm3_cti_gate::R`](R) reader structure"]
impl crate::Readable for FecCm3CtiGateSpec {}
#[doc = "`write(|w| ..)` method takes [`fec_cm3_cti_gate::W`](W) writer structure"]
impl crate::Writable for FecCm3CtiGateSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FEC_CM3_CTI_GATE to value 0"]
impl crate::Resettable for FecCm3CtiGateSpec {
    const RESET_VALUE: u32 = 0;
}
