#[doc = "Register `ETH_MACPPSCR` reader"]
pub struct R(crate::R<ETH_MACPPSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACPPSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACPPSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACPPSCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MACPPSCR` writer"]
pub struct W(crate::W<ETH_MACPPSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACPPSCR_SPEC>;
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
impl From<crate::W<ETH_MACPPSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACPPSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PPSCTRL` reader - PPS Output Frequency Control This field controls the frequency of the PPS output (eth_ptp_pps_out) signal. The default value of PPSCTRL is 0000, and the PPS output is 1 pulse (of width clk_ptp_i) every second. For other values of PPSCTRL, the PPS output becomes a generated clock of following frequencies: .. Note: In the binary rollover mode, the PPS output (eth_ptp_pps_out) has a duty cycle of 50 percent with these frequencies. In the digital rollover mode, the PPS output frequency is an average number. The actual clock is of different frequency that gets synchronized every second. For example: When PPSCTRL = 0001, the PPS (1 Hz) has a low period of 537 ms and a high period of 463 ms When PPSCTRL = 0010, the PPS (2 Hz) is a sequence of One clock of 50 percent duty cycle and 537 ms period Second clock of 463 ms period (268 ms low and 195 ms high) When PPSCTRL = 0011, the PPS (4 Hz) is a sequence of Three clocks of 50 percent duty cycle and 268 ms period Fourth clock of 195 ms period (134 ms low and 61 ms high) This behavior is because of the non-linear toggling of bits in the digital rollover mode in the ETH_MACSTNR register."]
pub type PPSCTRL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PPSCTRL` writer - PPS Output Frequency Control This field controls the frequency of the PPS output (eth_ptp_pps_out) signal. The default value of PPSCTRL is 0000, and the PPS output is 1 pulse (of width clk_ptp_i) every second. For other values of PPSCTRL, the PPS output becomes a generated clock of following frequencies: .. Note: In the binary rollover mode, the PPS output (eth_ptp_pps_out) has a duty cycle of 50 percent with these frequencies. In the digital rollover mode, the PPS output frequency is an average number. The actual clock is of different frequency that gets synchronized every second. For example: When PPSCTRL = 0001, the PPS (1 Hz) has a low period of 537 ms and a high period of 463 ms When PPSCTRL = 0010, the PPS (2 Hz) is a sequence of One clock of 50 percent duty cycle and 537 ms period Second clock of 463 ms period (268 ms low and 195 ms high) When PPSCTRL = 0011, the PPS (4 Hz) is a sequence of Three clocks of 50 percent duty cycle and 268 ms period Fourth clock of 195 ms period (134 ms low and 61 ms high) This behavior is because of the non-linear toggling of bits in the digital rollover mode in the ETH_MACSTNR register."]
pub type PPSCTRL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MACPPSCR_SPEC, u8, u8, 4, O>;
#[doc = "Field `PPSEN0` reader - Flexible PPS Output Mode Enable When this bit is set, PPSCTRL\\[3:0\\]
function as PPSCMD\\[3:0\\]. When this bit is reset, PPSCTRL\\[3:0\\]
function as PPSCTRL (Fixed PPS mode)."]
pub type PPSEN0_R = crate::BitReader<bool>;
#[doc = "Field `PPSEN0` writer - Flexible PPS Output Mode Enable When this bit is set, PPSCTRL\\[3:0\\]
function as PPSCMD\\[3:0\\]. When this bit is reset, PPSCTRL\\[3:0\\]
function as PPSCTRL (Fixed PPS mode)."]
pub type PPSEN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACPPSCR_SPEC, bool, O>;
#[doc = "Field `TRGTMODSEL0` reader - Target Time Register Mode for PPS Output This field indicates the Target Time registers (PPS target time seconds register (ETH_MACPPSTTSR) and PPS target time nanoseconds register (ETH_MACPPSTTNR)) mode for PPS output signal:"]
pub type TRGTMODSEL0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRGTMODSEL0` writer - Target Time Register Mode for PPS Output This field indicates the Target Time registers (PPS target time seconds register (ETH_MACPPSTTSR) and PPS target time nanoseconds register (ETH_MACPPSTTNR)) mode for PPS output signal:"]
pub type TRGTMODSEL0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ETH_MACPPSCR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:3 - PPS Output Frequency Control This field controls the frequency of the PPS output (eth_ptp_pps_out) signal. The default value of PPSCTRL is 0000, and the PPS output is 1 pulse (of width clk_ptp_i) every second. For other values of PPSCTRL, the PPS output becomes a generated clock of following frequencies: .. Note: In the binary rollover mode, the PPS output (eth_ptp_pps_out) has a duty cycle of 50 percent with these frequencies. In the digital rollover mode, the PPS output frequency is an average number. The actual clock is of different frequency that gets synchronized every second. For example: When PPSCTRL = 0001, the PPS (1 Hz) has a low period of 537 ms and a high period of 463 ms When PPSCTRL = 0010, the PPS (2 Hz) is a sequence of One clock of 50 percent duty cycle and 537 ms period Second clock of 463 ms period (268 ms low and 195 ms high) When PPSCTRL = 0011, the PPS (4 Hz) is a sequence of Three clocks of 50 percent duty cycle and 268 ms period Fourth clock of 195 ms period (134 ms low and 61 ms high) This behavior is because of the non-linear toggling of bits in the digital rollover mode in the ETH_MACSTNR register."]
    #[inline(always)]
    pub fn ppsctrl(&self) -> PPSCTRL_R {
        PPSCTRL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Flexible PPS Output Mode Enable When this bit is set, PPSCTRL\\[3:0\\]
function as PPSCMD\\[3:0\\]. When this bit is reset, PPSCTRL\\[3:0\\]
function as PPSCTRL (Fixed PPS mode)."]
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
    #[doc = "Bits 0:3 - PPS Output Frequency Control This field controls the frequency of the PPS output (eth_ptp_pps_out) signal. The default value of PPSCTRL is 0000, and the PPS output is 1 pulse (of width clk_ptp_i) every second. For other values of PPSCTRL, the PPS output becomes a generated clock of following frequencies: .. Note: In the binary rollover mode, the PPS output (eth_ptp_pps_out) has a duty cycle of 50 percent with these frequencies. In the digital rollover mode, the PPS output frequency is an average number. The actual clock is of different frequency that gets synchronized every second. For example: When PPSCTRL = 0001, the PPS (1 Hz) has a low period of 537 ms and a high period of 463 ms When PPSCTRL = 0010, the PPS (2 Hz) is a sequence of One clock of 50 percent duty cycle and 537 ms period Second clock of 463 ms period (268 ms low and 195 ms high) When PPSCTRL = 0011, the PPS (4 Hz) is a sequence of Three clocks of 50 percent duty cycle and 268 ms period Fourth clock of 195 ms period (134 ms low and 61 ms high) This behavior is because of the non-linear toggling of bits in the digital rollover mode in the ETH_MACSTNR register."]
    #[inline(always)]
    #[must_use]
    pub fn ppsctrl(&mut self) -> PPSCTRL_W<0> {
        PPSCTRL_W::new(self)
    }
    #[doc = "Bit 4 - Flexible PPS Output Mode Enable When this bit is set, PPSCTRL\\[3:0\\]
function as PPSCMD\\[3:0\\]. When this bit is reset, PPSCTRL\\[3:0\\]
function as PPSCTRL (Fixed PPS mode)."]
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
#[doc = "PPS control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macppscr](index.html) module"]
pub struct ETH_MACPPSCR_SPEC;
impl crate::RegisterSpec for ETH_MACPPSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_macppscr::R](R) reader structure"]
impl crate::Readable for ETH_MACPPSCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_macppscr::W](W) writer structure"]
impl crate::Writable for ETH_MACPPSCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETH_MACPPSCR to value 0"]
impl crate::Resettable for ETH_MACPPSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
