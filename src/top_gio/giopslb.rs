#[doc = "Register `GIOPSLB` reader"]
pub type R = crate::R<GiopslbSpec>;
#[doc = "Register `GIOPSLB` writer"]
pub type W = crate::W<GiopslbSpec>;
#[doc = "Field `GIOPSLB` reader - 7:0\\]
GIO pull select for port B"]
pub type GiopslbR = crate::FieldReader;
#[doc = "Field `GIOPSLB` writer - 7:0\\]
GIO pull select for port B"]
pub type GiopslbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NU36` reader - 31:8\\]
Reserved"]
pub type Nu36R = crate::FieldReader<u32>;
#[doc = "Field `NU36` writer - 31:8\\]
Reserved"]
pub type Nu36W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
GIO pull select for port B"]
    #[inline(always)]
    pub fn giopslb(&self) -> GiopslbR {
        GiopslbR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    pub fn nu36(&self) -> Nu36R {
        Nu36R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
GIO pull select for port B"]
    #[inline(always)]
    #[must_use]
    pub fn giopslb(&mut self) -> GiopslbW<GiopslbSpec> {
        GiopslbW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu36(&mut self) -> Nu36W<GiopslbSpec> {
        Nu36W::new(self, 8)
    }
}
#[doc = "GIO pul select for port B\n\nYou can [`read`](crate::Reg::read) this register and get [`giopslb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giopslb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GiopslbSpec;
impl crate::RegisterSpec for GiopslbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`giopslb::R`](R) reader structure"]
impl crate::Readable for GiopslbSpec {}
#[doc = "`write(|w| ..)` method takes [`giopslb::W`](W) writer structure"]
impl crate::Writable for GiopslbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIOPSLB to value 0"]
impl crate::Resettable for GiopslbSpec {
    const RESET_VALUE: u32 = 0;
}
