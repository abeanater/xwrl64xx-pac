#[doc = "Register `RTIWDSTATUS` reader"]
pub type R = crate::R<RtiwdstatusSpec>;
#[doc = "Register `RTIWDSTATUS` writer"]
pub type W = crate::W<RtiwdstatusSpec>;
#[doc = "Field `AWDST` reader - 0:0\\]
AWDST: Analog Watchdog Status. User and priviledge mode (read): 0 = AWD pin 0 ΓÇô> 1 threshold not exceeded 1 = AWD pin 0 ΓÇô> 1 threshold exceeded Priviledge mode (write): 0 = leaves the current value unchanged 1 = clears the bit to 0"]
pub type AwdstR = crate::BitReader;
#[doc = "Field `AWDST` writer - 0:0\\]
AWDST: Analog Watchdog Status. User and priviledge mode (read): 0 = AWD pin 0 ΓÇô> 1 threshold not exceeded 1 = AWD pin 0 ΓÇô> 1 threshold exceeded Priviledge mode (write): 0 = leaves the current value unchanged 1 = clears the bit to 0"]
pub type AwdstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DWDST` reader - 1:1\\]
DWDST: Digital Watchdog Status. This bit is effectively a copy of the END TIME VIOL status flag and is maintained for compatibility reasons. User and priviledge mode (read): 0 = DWD timeout period not expired 1 = DWD timeout period has expired Priviledge mode (write): 0 = leaves the current value unchanged 1 = clears the bit to 0"]
pub type DwdstR = crate::BitReader;
#[doc = "Field `DWDST` writer - 1:1\\]
DWDST: Digital Watchdog Status. This bit is effectively a copy of the END TIME VIOL status flag and is maintained for compatibility reasons. User and priviledge mode (read): 0 = DWD timeout period not expired 1 = DWD timeout period has expired Priviledge mode (write): 0 = leaves the current value unchanged 1 = clears the bit to 0"]
pub type DwdstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYST` reader - 2:2\\]
KEYST: Watchdog KeyStatus. This bit denotes a reset generated by a wrong key or a wrong key-sequence written to the RTIWDKEY register. User and priviledge mode (read): 0 = no wrong key or key-sequence written 1 = wrong key or key-sequence written to RTIWDKEY register Priviledge mode (write): 0 = leaves the current value unchanged 1 = clears the bit to 0"]
pub type KeystR = crate::BitReader;
#[doc = "Field `KEYST` writer - 2:2\\]
KEYST: Watchdog KeyStatus. This bit denotes a reset generated by a wrong key or a wrong key-sequence written to the RTIWDKEY register. User and priviledge mode (read): 0 = no wrong key or key-sequence written 1 = wrong key or key-sequence written to RTIWDKEY register Priviledge mode (write): 0 = leaves the current value unchanged 1 = clears the bit to 0"]
pub type KeystW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STARTTIMEVIOL` reader - 3:3\\]
START TIME VIOL: Windowed Watchdog Start Time Violation Status. This bit denotes whether the start-time defined by the windowed watchdog configuration has been violated. This indicates that the WWD was serviced before the service window was opened. User and priviledge mode (read): 0 = no start-time window violation has occurred. 1 = the start-time defined by the windowed watchdog configuration has been violated. Priviledge mode (write): 0 = leaves the current value unchanged. 1 = clears the bit to 0."]
pub type StarttimeviolR = crate::BitReader;
#[doc = "Field `STARTTIMEVIOL` writer - 3:3\\]
START TIME VIOL: Windowed Watchdog Start Time Violation Status. This bit denotes whether the start-time defined by the windowed watchdog configuration has been violated. This indicates that the WWD was serviced before the service window was opened. User and priviledge mode (read): 0 = no start-time window violation has occurred. 1 = the start-time defined by the windowed watchdog configuration has been violated. Priviledge mode (write): 0 = leaves the current value unchanged. 1 = clears the bit to 0."]
pub type StarttimeviolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDTIMEVIOL` reader - 4:4\\]
END TIME VIOL: Windowed Watchdog End Time Violation Status. This bit denotes whether the end-time defined by the windowed watchdog configuration has been violated. This bit is effectively a copy of the DWD ST status flag. User and priviledge mode (read): 0 = no end-time window violation has occurred. 1 = the end-time defined by the windowed watchdog configuration has been violated. Priviledge mode (write): 0 = leaves the current value unchanged. 1 = clears the bit to 0."]
pub type EndtimeviolR = crate::BitReader;
#[doc = "Field `ENDTIMEVIOL` writer - 4:4\\]
END TIME VIOL: Windowed Watchdog End Time Violation Status. This bit denotes whether the end-time defined by the windowed watchdog configuration has been violated. This bit is effectively a copy of the DWD ST status flag. User and priviledge mode (read): 0 = no end-time window violation has occurred. 1 = the end-time defined by the windowed watchdog configuration has been violated. Priviledge mode (write): 0 = leaves the current value unchanged. 1 = clears the bit to 0."]
pub type EndtimeviolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DWWD_ST` reader - 5:5\\]
DWWD ST: Windowed Watchdog Status. This bit denotes whether the time-window defined by the windowed watchdog configuration has been violated, or if a wrong key or key sequence was written to service the watchdog. User and priviledge mode (read): 0 = no time-window violation has occurred. 1 = a time-window violation has occurred. The watchdog will generate either a system reset or a non-maskable interrupt to the CPU in this case. Priviledge mode (write): 0 = leaves the current value unchanged. 1 = clears the bit to 0. This will also clear all other status flags in the RTIWDSTATUS register except for the AWD ST flag. Clearing of the status flags will deassert the non-maskable interrupt generated due to violation of the DWWD."]
pub type DwwdStR = crate::BitReader;
#[doc = "Field `DWWD_ST` writer - 5:5\\]
DWWD ST: Windowed Watchdog Status. This bit denotes whether the time-window defined by the windowed watchdog configuration has been violated, or if a wrong key or key sequence was written to service the watchdog. User and priviledge mode (read): 0 = no time-window violation has occurred. 1 = a time-window violation has occurred. The watchdog will generate either a system reset or a non-maskable interrupt to the CPU in this case. Priviledge mode (write): 0 = leaves the current value unchanged. 1 = clears the bit to 0. This will also clear all other status flags in the RTIWDSTATUS register except for the AWD ST flag. Clearing of the status flags will deassert the non-maskable interrupt generated due to violation of the DWWD."]
pub type DwwdStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED18` reader - 31:6\\]
Reserved. Reads return 0 and writes have no effect"]
pub type Reserved18R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED18` writer - 31:6\\]
Reserved. Reads return 0 and writes have no effect"]
pub type Reserved18W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
AWDST: Analog Watchdog Status. User and priviledge mode (read): 0 = AWD pin 0 ΓÇô> 1 threshold not exceeded 1 = AWD pin 0 ΓÇô> 1 threshold exceeded Priviledge mode (write): 0 = leaves the current value unchanged 1 = clears the bit to 0"]
    #[inline(always)]
    pub fn awdst(&self) -> AwdstR {
        AwdstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
DWDST: Digital Watchdog Status. This bit is effectively a copy of the END TIME VIOL status flag and is maintained for compatibility reasons. User and priviledge mode (read): 0 = DWD timeout period not expired 1 = DWD timeout period has expired Priviledge mode (write): 0 = leaves the current value unchanged 1 = clears the bit to 0"]
    #[inline(always)]
    pub fn dwdst(&self) -> DwdstR {
        DwdstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
KEYST: Watchdog KeyStatus. This bit denotes a reset generated by a wrong key or a wrong key-sequence written to the RTIWDKEY register. User and priviledge mode (read): 0 = no wrong key or key-sequence written 1 = wrong key or key-sequence written to RTIWDKEY register Priviledge mode (write): 0 = leaves the current value unchanged 1 = clears the bit to 0"]
    #[inline(always)]
    pub fn keyst(&self) -> KeystR {
        KeystR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
START TIME VIOL: Windowed Watchdog Start Time Violation Status. This bit denotes whether the start-time defined by the windowed watchdog configuration has been violated. This indicates that the WWD was serviced before the service window was opened. User and priviledge mode (read): 0 = no start-time window violation has occurred. 1 = the start-time defined by the windowed watchdog configuration has been violated. Priviledge mode (write): 0 = leaves the current value unchanged. 1 = clears the bit to 0."]
    #[inline(always)]
    pub fn starttimeviol(&self) -> StarttimeviolR {
        StarttimeviolR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
END TIME VIOL: Windowed Watchdog End Time Violation Status. This bit denotes whether the end-time defined by the windowed watchdog configuration has been violated. This bit is effectively a copy of the DWD ST status flag. User and priviledge mode (read): 0 = no end-time window violation has occurred. 1 = the end-time defined by the windowed watchdog configuration has been violated. Priviledge mode (write): 0 = leaves the current value unchanged. 1 = clears the bit to 0."]
    #[inline(always)]
    pub fn endtimeviol(&self) -> EndtimeviolR {
        EndtimeviolR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
DWWD ST: Windowed Watchdog Status. This bit denotes whether the time-window defined by the windowed watchdog configuration has been violated, or if a wrong key or key sequence was written to service the watchdog. User and priviledge mode (read): 0 = no time-window violation has occurred. 1 = a time-window violation has occurred. The watchdog will generate either a system reset or a non-maskable interrupt to the CPU in this case. Priviledge mode (write): 0 = leaves the current value unchanged. 1 = clears the bit to 0. This will also clear all other status flags in the RTIWDSTATUS register except for the AWD ST flag. Clearing of the status flags will deassert the non-maskable interrupt generated due to violation of the DWWD."]
    #[inline(always)]
    pub fn dwwd_st(&self) -> DwwdStR {
        DwwdStR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Reserved. Reads return 0 and writes have no effect"]
    #[inline(always)]
    pub fn reserved18(&self) -> Reserved18R {
        Reserved18R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
AWDST: Analog Watchdog Status. User and priviledge mode (read): 0 = AWD pin 0 ΓÇô> 1 threshold not exceeded 1 = AWD pin 0 ΓÇô> 1 threshold exceeded Priviledge mode (write): 0 = leaves the current value unchanged 1 = clears the bit to 0"]
    #[inline(always)]
    #[must_use]
    pub fn awdst(&mut self) -> AwdstW<RtiwdstatusSpec> {
        AwdstW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
DWDST: Digital Watchdog Status. This bit is effectively a copy of the END TIME VIOL status flag and is maintained for compatibility reasons. User and priviledge mode (read): 0 = DWD timeout period not expired 1 = DWD timeout period has expired Priviledge mode (write): 0 = leaves the current value unchanged 1 = clears the bit to 0"]
    #[inline(always)]
    #[must_use]
    pub fn dwdst(&mut self) -> DwdstW<RtiwdstatusSpec> {
        DwdstW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
KEYST: Watchdog KeyStatus. This bit denotes a reset generated by a wrong key or a wrong key-sequence written to the RTIWDKEY register. User and priviledge mode (read): 0 = no wrong key or key-sequence written 1 = wrong key or key-sequence written to RTIWDKEY register Priviledge mode (write): 0 = leaves the current value unchanged 1 = clears the bit to 0"]
    #[inline(always)]
    #[must_use]
    pub fn keyst(&mut self) -> KeystW<RtiwdstatusSpec> {
        KeystW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
START TIME VIOL: Windowed Watchdog Start Time Violation Status. This bit denotes whether the start-time defined by the windowed watchdog configuration has been violated. This indicates that the WWD was serviced before the service window was opened. User and priviledge mode (read): 0 = no start-time window violation has occurred. 1 = the start-time defined by the windowed watchdog configuration has been violated. Priviledge mode (write): 0 = leaves the current value unchanged. 1 = clears the bit to 0."]
    #[inline(always)]
    #[must_use]
    pub fn starttimeviol(&mut self) -> StarttimeviolW<RtiwdstatusSpec> {
        StarttimeviolW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
END TIME VIOL: Windowed Watchdog End Time Violation Status. This bit denotes whether the end-time defined by the windowed watchdog configuration has been violated. This bit is effectively a copy of the DWD ST status flag. User and priviledge mode (read): 0 = no end-time window violation has occurred. 1 = the end-time defined by the windowed watchdog configuration has been violated. Priviledge mode (write): 0 = leaves the current value unchanged. 1 = clears the bit to 0."]
    #[inline(always)]
    #[must_use]
    pub fn endtimeviol(&mut self) -> EndtimeviolW<RtiwdstatusSpec> {
        EndtimeviolW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
DWWD ST: Windowed Watchdog Status. This bit denotes whether the time-window defined by the windowed watchdog configuration has been violated, or if a wrong key or key sequence was written to service the watchdog. User and priviledge mode (read): 0 = no time-window violation has occurred. 1 = a time-window violation has occurred. The watchdog will generate either a system reset or a non-maskable interrupt to the CPU in this case. Priviledge mode (write): 0 = leaves the current value unchanged. 1 = clears the bit to 0. This will also clear all other status flags in the RTIWDSTATUS register except for the AWD ST flag. Clearing of the status flags will deassert the non-maskable interrupt generated due to violation of the DWWD."]
    #[inline(always)]
    #[must_use]
    pub fn dwwd_st(&mut self) -> DwwdStW<RtiwdstatusSpec> {
        DwwdStW::new(self, 5)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Reserved. Reads return 0 and writes have no effect"]
    #[inline(always)]
    #[must_use]
    pub fn reserved18(&mut self) -> Reserved18W<RtiwdstatusSpec> {
        Reserved18W::new(self, 6)
    }
}
#[doc = "Watchdog Status reflects the status of Analog and Digital Watchdog\n\nYou can [`read`](crate::Reg::read) this register and get [`rtiwdstatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtiwdstatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtiwdstatusSpec;
impl crate::RegisterSpec for RtiwdstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtiwdstatus::R`](R) reader structure"]
impl crate::Readable for RtiwdstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`rtiwdstatus::W`](W) writer structure"]
impl crate::Writable for RtiwdstatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTIWDSTATUS to value 0"]
impl crate::Resettable for RtiwdstatusSpec {
    const RESET_VALUE: u32 = 0;
}
