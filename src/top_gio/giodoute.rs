#[doc = "Register `GIODOUTE` reader"]
pub type R = crate::R<GiodouteSpec>;
#[doc = "Register `GIODOUTE` writer"]
pub type W = crate::W<GiodouteSpec>;
#[doc = "Field `GIODOUTE` reader - 7:0\\]
GIO data output for pins in port E"]
pub type GiodouteR = crate::FieldReader;
#[doc = "Field `GIODOUTE` writer - 7:0\\]
GIO data output for pins in port E"]
pub type GiodouteW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NU21` reader - 31:8\\]
Reserved"]
pub type Nu21R = crate::FieldReader<u32>;
#[doc = "Field `NU21` writer - 31:8\\]
Reserved"]
pub type Nu21W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
GIO data output for pins in port E"]
    #[inline(always)]
    pub fn giodoute(&self) -> GiodouteR {
        GiodouteR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    pub fn nu21(&self) -> Nu21R {
        Nu21R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
GIO data output for pins in port E"]
    #[inline(always)]
    #[must_use]
    pub fn giodoute(&mut self) -> GiodouteW<GiodouteSpec> {
        GiodouteW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu21(&mut self) -> Nu21W<GiodouteSpec> {
        Nu21W::new(self, 8)
    }
}
#[doc = "GIO data output for pins in port E\n\nYou can [`read`](crate::Reg::read) this register and get [`giodoute::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`giodoute::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GiodouteSpec;
impl crate::RegisterSpec for GiodouteSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`giodoute::R`](R) reader structure"]
impl crate::Readable for GiodouteSpec {}
#[doc = "`write(|w| ..)` method takes [`giodoute::W`](W) writer structure"]
impl crate::Writable for GiodouteSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIODOUTE to value 0"]
impl crate::Resettable for GiodouteSpec {
    const RESET_VALUE: u32 = 0;
}
