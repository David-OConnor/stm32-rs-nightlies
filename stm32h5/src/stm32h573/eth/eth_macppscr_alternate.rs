#[doc = "Register `ETH_MACPPSCR_ALTERNATE` reader"]
pub struct R(crate::R<ETH_MACPPSCR_ALTERNATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACPPSCR_ALTERNATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACPPSCR_ALTERNATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACPPSCR_ALTERNATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MACPPSCR_ALTERNATE` writer"]
pub struct W(crate::W<ETH_MACPPSCR_ALTERNATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACPPSCR_ALTERNATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<ETH_MACPPSCR_ALTERNATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACPPSCR_ALTERNATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PPSCMD` reader - Flexible PPS Output (eth_ptp_pps_out) Control Programming these bits with a non-zero value instructs the MAC to initiate an event. When the command is transferred or synchronized to the PTP clock domain, these bits get cleared automatically. The software should ensure that these bits are programmed only when they are ‘all zero’. The following list describes the values of PPSCMD0: This command generates single pulse rising at the start point defined in Target Time Registers (register 455 and 456) and of a duration defined in the PPS Width Register. This command generates the train of pulses rising at the start point defined in the Target Time Registers and of a duration defined in the PPS Width Register and repeated at interval defined in the PPS Interval Register. By default, the PPS pulse train is free-running unless stopped by the 'Stop Pulse train at time' or 'Stop Pulse Train immediately' commands. This command cancels the START Single Pulse and START Pulse Train commands if the system time has not crossed the programmed start time. This command stops the train of pulses initiated by the START Pulse Train command (PPSCMD\\[3:0\\]
= 0010) after the time programmed in the Target Time registers elapses. This command immediately stops the train of pulses initiated by the START Pulse Train command (PPSCMD\\[3:0\\]
= 0010). This command cancels the STOP pulse train at time command if the programmed stop time has not elapsed. The PPS pulse train becomes free-running on the successful execution of this command. 0111 to 1111: Reserved, must not be used"]
pub type PPSCMD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PPSCMD` writer - Flexible PPS Output (eth_ptp_pps_out) Control Programming these bits with a non-zero value instructs the MAC to initiate an event. When the command is transferred or synchronized to the PTP clock domain, these bits get cleared automatically. The software should ensure that these bits are programmed only when they are ‘all zero’. The following list describes the values of PPSCMD0: This command generates single pulse rising at the start point defined in Target Time Registers (register 455 and 456) and of a duration defined in the PPS Width Register. This command generates the train of pulses rising at the start point defined in the Target Time Registers and of a duration defined in the PPS Width Register and repeated at interval defined in the PPS Interval Register. By default, the PPS pulse train is free-running unless stopped by the 'Stop Pulse train at time' or 'Stop Pulse Train immediately' commands. This command cancels the START Single Pulse and START Pulse Train commands if the system time has not crossed the programmed start time. This command stops the train of pulses initiated by the START Pulse Train command (PPSCMD\\[3:0\\]
= 0010) after the time programmed in the Target Time registers elapses. This command immediately stops the train of pulses initiated by the START Pulse Train command (PPSCMD\\[3:0\\]
= 0010). This command cancels the STOP pulse train at time command if the programmed stop time has not elapsed. The PPS pulse train becomes free-running on the successful execution of this command. 0111 to 1111: Reserved, must not be used"]
pub type PPSCMD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ETH_MACPPSCR_ALTERNATE_SPEC, u8, u8, 4, O>;
#[doc = "Field `PPSEN0` reader - Flexible PPS Output Mode Enable When this bit is set, Bits\\[3:0\\]
function as PPSCMD\\[3:0\\]. When this bit is reset, Bits\\[3:0\\]
function as PPSCTRL(Fixed PPS mode)."]
pub type PPSEN0_R = crate::BitReader<bool>;
#[doc = "Field `PPSEN0` writer - Flexible PPS Output Mode Enable When this bit is set, Bits\\[3:0\\]
function as PPSCMD\\[3:0\\]. When this bit is reset, Bits\\[3:0\\]
function as PPSCTRL(Fixed PPS mode)."]
pub type PPSEN0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ETH_MACPPSCR_ALTERNATE_SPEC, bool, O>;
#[doc = "Field `TRGTMODSEL0` reader - Target Time Register Mode for PPS Output This field indicates the Target Time registers (PPS target time seconds register (ETH_MACPPSTTSR) and PPS target time nanoseconds register (ETH_MACPPSTTNR)) mode for PPS output signal:"]
pub type TRGTMODSEL0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRGTMODSEL0` writer - Target Time Register Mode for PPS Output This field indicates the Target Time registers (PPS target time seconds register (ETH_MACPPSTTSR) and PPS target time nanoseconds register (ETH_MACPPSTTNR)) mode for PPS output signal:"]
pub type TRGTMODSEL0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ETH_MACPPSCR_ALTERNATE_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:3 - Flexible PPS Output (eth_ptp_pps_out) Control Programming these bits with a non-zero value instructs the MAC to initiate an event. When the command is transferred or synchronized to the PTP clock domain, these bits get cleared automatically. The software should ensure that these bits are programmed only when they are ‘all zero’. The following list describes the values of PPSCMD0: This command generates single pulse rising at the start point defined in Target Time Registers (register 455 and 456) and of a duration defined in the PPS Width Register. This command generates the train of pulses rising at the start point defined in the Target Time Registers and of a duration defined in the PPS Width Register and repeated at interval defined in the PPS Interval Register. By default, the PPS pulse train is free-running unless stopped by the 'Stop Pulse train at time' or 'Stop Pulse Train immediately' commands. This command cancels the START Single Pulse and START Pulse Train commands if the system time has not crossed the programmed start time. This command stops the train of pulses initiated by the START Pulse Train command (PPSCMD\\[3:0\\]
= 0010) after the time programmed in the Target Time registers elapses. This command immediately stops the train of pulses initiated by the START Pulse Train command (PPSCMD\\[3:0\\]
= 0010). This command cancels the STOP pulse train at time command if the programmed stop time has not elapsed. The PPS pulse train becomes free-running on the successful execution of this command. 0111 to 1111: Reserved, must not be used"]
    #[inline(always)]
    pub fn ppscmd(&self) -> PPSCMD_R {
        PPSCMD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Flexible PPS Output Mode Enable When this bit is set, Bits\\[3:0\\]
function as PPSCMD\\[3:0\\]. When this bit is reset, Bits\\[3:0\\]
function as PPSCTRL(Fixed PPS mode)."]
    #[inline(always)]
    pub fn ppsen0(&self) -> PPSEN0_R {
        PPSEN0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Target Time Register Mode for PPS Output This field indicates the Target Time registers (PPS target time seconds register (ETH_MACPPSTTSR) and PPS target time nanoseconds register (ETH_MACPPSTTNR)) mode for PPS output signal:"]
    #[inline(always)]
    pub fn trgtmodsel0(&self) -> TRGTMODSEL0_R {
        TRGTMODSEL0_R::new(((self.bits >> 5) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Flexible PPS Output (eth_ptp_pps_out) Control Programming these bits with a non-zero value instructs the MAC to initiate an event. When the command is transferred or synchronized to the PTP clock domain, these bits get cleared automatically. The software should ensure that these bits are programmed only when they are ‘all zero’. The following list describes the values of PPSCMD0: This command generates single pulse rising at the start point defined in Target Time Registers (register 455 and 456) and of a duration defined in the PPS Width Register. This command generates the train of pulses rising at the start point defined in the Target Time Registers and of a duration defined in the PPS Width Register and repeated at interval defined in the PPS Interval Register. By default, the PPS pulse train is free-running unless stopped by the 'Stop Pulse train at time' or 'Stop Pulse Train immediately' commands. This command cancels the START Single Pulse and START Pulse Train commands if the system time has not crossed the programmed start time. This command stops the train of pulses initiated by the START Pulse Train command (PPSCMD\\[3:0\\]
= 0010) after the time programmed in the Target Time registers elapses. This command immediately stops the train of pulses initiated by the START Pulse Train command (PPSCMD\\[3:0\\]
= 0010). This command cancels the STOP pulse train at time command if the programmed stop time has not elapsed. The PPS pulse train becomes free-running on the successful execution of this command. 0111 to 1111: Reserved, must not be used"]
    #[inline(always)]
    #[must_use]
    pub fn ppscmd(&mut self) -> PPSCMD_W<0> {
        PPSCMD_W::new(self)
    }
    #[doc = "Bit 4 - Flexible PPS Output Mode Enable When this bit is set, Bits\\[3:0\\]
function as PPSCMD\\[3:0\\]. When this bit is reset, Bits\\[3:0\\]
function as PPSCTRL(Fixed PPS mode)."]
    #[inline(always)]
    #[must_use]
    pub fn ppsen0(&mut self) -> PPSEN0_W<4> {
        PPSEN0_W::new(self)
    }
    #[doc = "Bits 5:6 - Target Time Register Mode for PPS Output This field indicates the Target Time registers (PPS target time seconds register (ETH_MACPPSTTSR) and PPS target time nanoseconds register (ETH_MACPPSTTNR)) mode for PPS output signal:"]
    #[inline(always)]
    #[must_use]
    pub fn trgtmodsel0(&mut self) -> TRGTMODSEL0_W<5> {
        TRGTMODSEL0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PPS control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macppscr_alternate](index.html) module"]
pub struct ETH_MACPPSCR_ALTERNATE_SPEC;
impl crate::RegisterSpec for ETH_MACPPSCR_ALTERNATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_macppscr_alternate::R](R) reader structure"]
impl crate::Readable for ETH_MACPPSCR_ALTERNATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_macppscr_alternate::W](W) writer structure"]
impl crate::Writable for ETH_MACPPSCR_ALTERNATE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETH_MACPPSCR_ALTERNATE to value 0"]
impl crate::Resettable for ETH_MACPPSCR_ALTERNATE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
