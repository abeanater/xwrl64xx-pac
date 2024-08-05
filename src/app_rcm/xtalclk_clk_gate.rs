#[doc = "Register `XTALCLK_CLK_GATE` reader"]
pub type R = crate::R<XtalclkClkGateSpec>;
#[doc = "Register `XTALCLK_CLK_GATE` writer"]
pub type W = crate::W<XtalclkClkGateSpec>;
#[doc = "Field `XTALCLK_CLK_GATE` reader - 2:0\\]
RESERVED"]
pub type XtalclkClkGateR = crate::FieldReader;
#[doc = "Field `XTALCLK_CLK_GATE` writer - 2:0\\]
RESERVED"]
pub type XtalclkClkGateW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
RESERVED"]
    #[inline(always)]
    pub fn xtalclk_clk_gate(&self) -> XtalclkClkGateR {
        XtalclkClkGateR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
RESERVED"]
    #[inline(always)]
    #[must_use]
    pub fn xtalclk_clk_gate(&mut self) -> XtalclkClkGateW<XtalclkClkGateSpec> {
        XtalclkClkGateW::new(self, 0)
    }
}
#[doc = "XTALCLK_CLK_GATE\n\nYou can [`read`](crate::Reg::read) this register and get [`xtalclk_clk_gate::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtalclk_clk_gate::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XtalclkClkGateSpec;
impl crate::RegisterSpec for XtalclkClkGateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xtalclk_clk_gate::R`](R) reader structure"]
impl crate::Readable for XtalclkClkGateSpec {}
#[doc = "`write(|w| ..)` method takes [`xtalclk_clk_gate::W`](W) writer structure"]
impl crate::Writable for XtalclkClkGateSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets XTALCLK_CLK_GATE to value 0"]
impl crate::Resettable for XtalclkClkGateSpec {
    const RESET_VALUE: u32 = 0;
}
