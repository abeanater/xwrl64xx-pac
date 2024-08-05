#[doc = "Register `GIOCLRE` reader"]
pub type R = crate::R<GioclreSpec>;
#[doc = "Register `GIOCLRE` writer"]
pub type W = crate::W<GioclreSpec>;
#[doc = "Field `GIODCLRE` reader - 7:0\\]
GIO data clear for port E"]
pub type GiodclreR = crate::FieldReader;
#[doc = "Field `GIODCLRE` writer - 7:0\\]
GIO data clear for port E"]
pub type GiodclreW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NU33` reader - 31:8\\]
Reserved"]
pub type Nu33R = crate::FieldReader<u32>;
#[doc = "Field `NU33` writer - 31:8\\]
Reserved"]
pub type Nu33W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
GIO data clear for port E"]
    #[inline(always)]
    pub fn giodclre(&self) -> GiodclreR {
        GiodclreR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    pub fn nu33(&self) -> Nu33R {
        Nu33R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
GIO data clear for port E"]
    #[inline(always)]
    #[must_use]
    pub fn giodclre(&mut self) -> GiodclreW<GioclreSpec> {
        GiodclreW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu33(&mut self) -> Nu33W<GioclreSpec> {
        Nu33W::new(self, 8)
    }
}
#[doc = "GIO data clear for port E\n\nYou can [`read`](crate::Reg::read) this register and get [`gioclre::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gioclre::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GioclreSpec;
impl crate::RegisterSpec for GioclreSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gioclre::R`](R) reader structure"]
impl crate::Readable for GioclreSpec {}
#[doc = "`write(|w| ..)` method takes [`gioclre::W`](W) writer structure"]
impl crate::Writable for GioclreSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIOCLRE to value 0"]
impl crate::Resettable for GioclreSpec {
    const RESET_VALUE: u32 = 0;
}
