#[doc = "Register `DC_ACC3I_LSB` reader"]
pub type R = crate::R<DcAcc3iLsbSpec>;
#[doc = "Register `DC_ACC3I_LSB` writer"]
pub type W = crate::W<DcAcc3iLsbSpec>;
#[doc = "Field `DC_ACC3I_LSB` reader - 31:0\\]
This register provides the LSB 32 bits value of DC accumulator I channel for bcnt=2"]
pub type DcAcc3iLsbR = crate::FieldReader<u32>;
#[doc = "Field `DC_ACC3I_LSB` writer - 31:0\\]
This register provides the LSB 32 bits value of DC accumulator I channel for bcnt=2"]
pub type DcAcc3iLsbW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This register provides the LSB 32 bits value of DC accumulator I channel for bcnt=2"]
    #[inline(always)]
    pub fn dc_acc3i_lsb(&self) -> DcAcc3iLsbR {
        DcAcc3iLsbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This register provides the LSB 32 bits value of DC accumulator I channel for bcnt=2"]
    #[inline(always)]
    #[must_use]
    pub fn dc_acc3i_lsb(&mut self) -> DcAcc3iLsbW<DcAcc3iLsbSpec> {
        DcAcc3iLsbW::new(self, 0)
    }
}
#[doc = "DC_ACC3I_LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_acc3i_lsb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_acc3i_lsb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcAcc3iLsbSpec;
impl crate::RegisterSpec for DcAcc3iLsbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dc_acc3i_lsb::R`](R) reader structure"]
impl crate::Readable for DcAcc3iLsbSpec {}
#[doc = "`write(|w| ..)` method takes [`dc_acc3i_lsb::W`](W) writer structure"]
impl crate::Writable for DcAcc3iLsbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DC_ACC3I_LSB to value 0"]
impl crate::Resettable for DcAcc3iLsbSpec {
    const RESET_VALUE: u32 = 0;
}
