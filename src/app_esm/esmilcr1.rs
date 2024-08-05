#[doc = "Register `ESMILCR1` reader"]
pub type R = crate::R<Esmilcr1Spec>;
#[doc = "Register `ESMILCR1` writer"]
pub type W = crate::W<Esmilcr1Spec>;
#[doc = "Field `INTLVLCLR` reader - 31:0\\]
Clear Interrupt Priority. Read in User and Privileged mode. Write in Privileged mode only. 0 Read: Interrupt of channel x is mapped to low level interrupt line. Write: Leaves the bit and the corresponding set bit in the ESMILSR1 register unchanged. 1 Read: Interrupt of channel x is mapped to high level interrupt line. Write: Maps interrupt of channel x to low level interrupt line and clears the corresponding set bit in the ESMILSR1 register."]
pub type IntlvlclrR = crate::FieldReader<u32>;
#[doc = "Field `INTLVLCLR` writer - 31:0\\]
Clear Interrupt Priority. Read in User and Privileged mode. Write in Privileged mode only. 0 Read: Interrupt of channel x is mapped to low level interrupt line. Write: Leaves the bit and the corresponding set bit in the ESMILSR1 register unchanged. 1 Read: Interrupt of channel x is mapped to high level interrupt line. Write: Maps interrupt of channel x to low level interrupt line and clears the corresponding set bit in the ESMILSR1 register."]
pub type IntlvlclrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Clear Interrupt Priority. Read in User and Privileged mode. Write in Privileged mode only. 0 Read: Interrupt of channel x is mapped to low level interrupt line. Write: Leaves the bit and the corresponding set bit in the ESMILSR1 register unchanged. 1 Read: Interrupt of channel x is mapped to high level interrupt line. Write: Maps interrupt of channel x to low level interrupt line and clears the corresponding set bit in the ESMILSR1 register."]
    #[inline(always)]
    pub fn intlvlclr(&self) -> IntlvlclrR {
        IntlvlclrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Clear Interrupt Priority. Read in User and Privileged mode. Write in Privileged mode only. 0 Read: Interrupt of channel x is mapped to low level interrupt line. Write: Leaves the bit and the corresponding set bit in the ESMILSR1 register unchanged. 1 Read: Interrupt of channel x is mapped to high level interrupt line. Write: Maps interrupt of channel x to low level interrupt line and clears the corresponding set bit in the ESMILSR1 register."]
    #[inline(always)]
    #[must_use]
    pub fn intlvlclr(&mut self) -> IntlvlclrW<Esmilcr1Spec> {
        IntlvlclrW::new(self, 0)
    }
}
#[doc = "Interrupt Level Clear/Status Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`esmilcr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esmilcr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Esmilcr1Spec;
impl crate::RegisterSpec for Esmilcr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`esmilcr1::R`](R) reader structure"]
impl crate::Readable for Esmilcr1Spec {}
#[doc = "`write(|w| ..)` method takes [`esmilcr1::W`](W) writer structure"]
impl crate::Writable for Esmilcr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ESMILCR1 to value 0"]
impl crate::Resettable for Esmilcr1Spec {
    const RESET_VALUE: u32 = 0;
}