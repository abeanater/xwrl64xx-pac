#[doc = "Register `INTF_CHIRP_ZEROCOUNT` reader"]
pub type R = crate::R<IntfChirpZerocountSpec>;
#[doc = "Register `INTF_CHIRP_ZEROCOUNT` writer"]
pub type W = crate::W<IntfChirpZerocountSpec>;
#[doc = "Field `INTF_CHIRP_ZEROCOUNT` reader - 11:0\\]
Number of samples that exceeded the threshold in a chirp"]
pub type IntfChirpZerocountR = crate::FieldReader<u16>;
#[doc = "Field `INTF_CHIRP_ZEROCOUNT` writer - 11:0\\]
Number of samples that exceeded the threshold in a chirp"]
pub type IntfChirpZerocountW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `NU1` reader - "]
pub type Nu1R = crate::FieldReader<u32>;
#[doc = "Field `NU1` writer - "]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Number of samples that exceeded the threshold in a chirp"]
    #[inline(always)]
    pub fn intf_chirp_zerocount(&self) -> IntfChirpZerocountR {
        IntfChirpZerocountR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:31"]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
Number of samples that exceeded the threshold in a chirp"]
    #[inline(always)]
    #[must_use]
    pub fn intf_chirp_zerocount(&mut self) -> IntfChirpZerocountW<IntfChirpZerocountSpec> {
        IntfChirpZerocountW::new(self, 0)
    }
    #[doc = "Bits 12:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<IntfChirpZerocountSpec> {
        Nu1W::new(self, 12)
    }
}
#[doc = "INTF_CHIRP_ZEROCOUNT\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_chirp_zerocount::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_chirp_zerocount::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntfChirpZerocountSpec;
impl crate::RegisterSpec for IntfChirpZerocountSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf_chirp_zerocount::R`](R) reader structure"]
impl crate::Readable for IntfChirpZerocountSpec {}
#[doc = "`write(|w| ..)` method takes [`intf_chirp_zerocount::W`](W) writer structure"]
impl crate::Writable for IntfChirpZerocountSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTF_CHIRP_ZEROCOUNT to value 0"]
impl crate::Resettable for IntfChirpZerocountSpec {
    const RESET_VALUE: u32 = 0;
}
