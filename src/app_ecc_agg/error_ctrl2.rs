#[doc = "Register `ERROR_CTRL2` reader"]
pub type R = crate::R<ErrorCtrl2Spec>;
#[doc = "Register `ERROR_CTRL2` writer"]
pub type W = crate::W<ErrorCtrl2Spec>;
#[doc = "Field `ECC_BIT1` reader - 15:0\\]
Data bit that needs to be flipped when force_sec is set - (RW )"]
pub type EccBit1R = crate::FieldReader<u16>;
#[doc = "Field `ECC_BIT1` writer - 15:0\\]
Data bit that needs to be flipped when force_sec is set - (RW )"]
pub type EccBit1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ECC_BIT2` reader - 31:16\\]
Data bit that needs to be flipped if double bit error needs to be forced - (RW )"]
pub type EccBit2R = crate::FieldReader<u16>;
#[doc = "Field `ECC_BIT2` writer - 31:16\\]
Data bit that needs to be flipped if double bit error needs to be forced - (RW )"]
pub type EccBit2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Data bit that needs to be flipped when force_sec is set - (RW )"]
    #[inline(always)]
    pub fn ecc_bit1(&self) -> EccBit1R {
        EccBit1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Data bit that needs to be flipped if double bit error needs to be forced - (RW )"]
    #[inline(always)]
    pub fn ecc_bit2(&self) -> EccBit2R {
        EccBit2R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Data bit that needs to be flipped when force_sec is set - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn ecc_bit1(&mut self) -> EccBit1W<ErrorCtrl2Spec> {
        EccBit1W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Data bit that needs to be flipped if double bit error needs to be forced - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn ecc_bit2(&mut self) -> EccBit2W<ErrorCtrl2Spec> {
        EccBit2W::new(self, 16)
    }
}
#[doc = "ECC Error Control2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`error_ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`error_ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrorCtrl2Spec;
impl crate::RegisterSpec for ErrorCtrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`error_ctrl2::R`](R) reader structure"]
impl crate::Readable for ErrorCtrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`error_ctrl2::W`](W) writer structure"]
impl crate::Writable for ErrorCtrl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ERROR_CTRL2 to value 0"]
impl crate::Resettable for ErrorCtrl2Spec {
    const RESET_VALUE: u32 = 0;
}