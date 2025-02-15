#[doc = "Register `TIM15_BDTR` reader"]
pub struct R(crate::R<TIM15_BDTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM15_BDTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM15_BDTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM15_BDTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM15_BDTR` writer"]
pub struct W(crate::W<TIM15_BDTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM15_BDTR_SPEC>;
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
impl From<crate::W<TIM15_BDTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM15_BDTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTG` reader - Dead-time generator setup This bit-field defines the duration of the dead-time inserted between the complementary outputs. DT correspond to this duration. DTG\\[7:5\\]=0xx => DT=DTG\\[7:0\\]x tdtg with tdtg=tDTS DTG\\[7:5\\]=10x => DT=(64+DTG\\[5:0\\])xtdtg with Tdtg=2xtDTS DTG\\[7:5\\]=110 => DT=(32+DTG\\[4:0\\])xtdtg with Tdtg=8xtDTS DTG\\[7:5\\]=111 => DT=(32+DTG\\[4:0\\])xtdtg with Tdtg=16xtDTS Example if TDTS=125ns (8MHz), dead-time possible values are: 0 to 15875 ns by 125 ns steps, 16 �s to 31750 ns by 250 ns steps, 32 �s to 63 �s by 1 �s steps, 64 �s to 126 �s by 2 �s steps Note: This bit-field can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIM15_BDTR register)."]
pub type DTG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DTG` writer - Dead-time generator setup This bit-field defines the duration of the dead-time inserted between the complementary outputs. DT correspond to this duration. DTG\\[7:5\\]=0xx => DT=DTG\\[7:0\\]x tdtg with tdtg=tDTS DTG\\[7:5\\]=10x => DT=(64+DTG\\[5:0\\])xtdtg with Tdtg=2xtDTS DTG\\[7:5\\]=110 => DT=(32+DTG\\[4:0\\])xtdtg with Tdtg=8xtDTS DTG\\[7:5\\]=111 => DT=(32+DTG\\[4:0\\])xtdtg with Tdtg=16xtDTS Example if TDTS=125ns (8MHz), dead-time possible values are: 0 to 15875 ns by 125 ns steps, 16 �s to 31750 ns by 250 ns steps, 32 �s to 63 �s by 1 �s steps, 64 �s to 126 �s by 2 �s steps Note: This bit-field can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIM15_BDTR register)."]
pub type DTG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM15_BDTR_SPEC, u8, u8, 8, O>;
#[doc = "Field `LOCK` reader - Lock configuration These bits offer a write protection against software errors. Note: The LOCK bits can be written only once after the reset. Once the TIM15_BDTR register has been written, their content is frozen until the next reset."]
pub type LOCK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LOCK` writer - Lock configuration These bits offer a write protection against software errors. Note: The LOCK bits can be written only once after the reset. Once the TIM15_BDTR register has been written, their content is frozen until the next reset."]
pub type LOCK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM15_BDTR_SPEC, u8, u8, 2, O>;
#[doc = "Field `OSSI` reader - Off-state selection for Idle mode This bit is used when MOE=0 on channels configured as outputs. See tim_ocx/tim_ocxn enable description for more details (capture/compare enable register (TIM15_CCER) on page 1985). Note: This bit can not be modified as soon as the LOCK level 2 has been programmed (LOCK bits in TIM15_BDTR register)."]
pub type OSSI_R = crate::BitReader<bool>;
#[doc = "Field `OSSI` writer - Off-state selection for Idle mode This bit is used when MOE=0 on channels configured as outputs. See tim_ocx/tim_ocxn enable description for more details (capture/compare enable register (TIM15_CCER) on page 1985). Note: This bit can not be modified as soon as the LOCK level 2 has been programmed (LOCK bits in TIM15_BDTR register)."]
pub type OSSI_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM15_BDTR_SPEC, bool, O>;
#[doc = "Field `OSSR` reader - Off-state selection for Run mode This bit is used when MOE=1 on channels that have a complementary output which are configured as outputs. OSSR is not implemented if no complementary output is implemented in the timer. See tim_ocx/tim_ocxn enable description for more details (capture/compare enable register (TIM15_CCER) on page 1985). Note: This bit can not be modified as soon as the LOCK level 2 has been programmed (LOCK bits in TIM15_BDTR register)."]
pub type OSSR_R = crate::BitReader<bool>;
#[doc = "Field `OSSR` writer - Off-state selection for Run mode This bit is used when MOE=1 on channels that have a complementary output which are configured as outputs. OSSR is not implemented if no complementary output is implemented in the timer. See tim_ocx/tim_ocxn enable description for more details (capture/compare enable register (TIM15_CCER) on page 1985). Note: This bit can not be modified as soon as the LOCK level 2 has been programmed (LOCK bits in TIM15_BDTR register)."]
pub type OSSR_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM15_BDTR_SPEC, bool, O>;
#[doc = "Field `BKE` reader - Break enable 1; Break inputs (tim_brk and tim_sys_brk clock failure event) enabled This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIM15_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
pub type BKE_R = crate::BitReader<bool>;
#[doc = "Field `BKE` writer - Break enable 1; Break inputs (tim_brk and tim_sys_brk clock failure event) enabled This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIM15_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
pub type BKE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM15_BDTR_SPEC, bool, O>;
#[doc = "Field `BKP` reader - Break polarity Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIM15_BDTR register). Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
pub type BKP_R = crate::BitReader<bool>;
#[doc = "Field `BKP` writer - Break polarity Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIM15_BDTR register). Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
pub type BKP_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM15_BDTR_SPEC, bool, O>;
#[doc = "Field `AOE` reader - Automatic output enable Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIM15_BDTR register)."]
pub type AOE_R = crate::BitReader<bool>;
#[doc = "Field `AOE` writer - Automatic output enable Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIM15_BDTR register)."]
pub type AOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM15_BDTR_SPEC, bool, O>;
#[doc = "Field `MOE` reader - Main output enable This bit is cleared asynchronously by hardware as soon as the tim_brk input is active. It is set by software or automatically depending on the AOE bit. It is acting only on the channels which are configured in output. See tim_ocx/tim_ocxn enable description for more details (capture/compare enable register (TIM15_CCER) on page 1985)."]
pub type MOE_R = crate::BitReader<bool>;
#[doc = "Field `MOE` writer - Main output enable This bit is cleared asynchronously by hardware as soon as the tim_brk input is active. It is set by software or automatically depending on the AOE bit. It is acting only on the channels which are configured in output. See tim_ocx/tim_ocxn enable description for more details (capture/compare enable register (TIM15_CCER) on page 1985)."]
pub type MOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM15_BDTR_SPEC, bool, O>;
#[doc = "Field `BKF` reader - Break filter This bit-field defines the frequency used to sample the tim_brk input signal and the length of the digital filter applied to tim_brk. The digital filter is made of an event counter in which N events are needed to validate a transition on the output: Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIM15_BDTR register)."]
pub type BKF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BKF` writer - Break filter This bit-field defines the frequency used to sample the tim_brk input signal and the length of the digital filter applied to tim_brk. The digital filter is made of an event counter in which N events are needed to validate a transition on the output: Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIM15_BDTR register)."]
pub type BKF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM15_BDTR_SPEC, u8, u8, 4, O>;
#[doc = "Field `BKDSRM` reader - Break disarm This bit is cleared by hardware when no break source is active. The BKDSRM bit must be set by software to release the bidirectional output control (open-drain output in Hi-Z state) and then be polled until it is reset by hardware, indicating that the fault condition has disappeared. Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
pub type BKDSRM_R = crate::BitReader<bool>;
#[doc = "Field `BKDSRM` writer - Break disarm This bit is cleared by hardware when no break source is active. The BKDSRM bit must be set by software to release the bidirectional output control (open-drain output in Hi-Z state) and then be polled until it is reset by hardware, indicating that the fault condition has disappeared. Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
pub type BKDSRM_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM15_BDTR_SPEC, bool, O>;
#[doc = "Field `BKBID` reader - Break bidirectional In the bidirectional mode (BKBID bit set to 1), the break input is configured both in input mode and in open drain output mode. Any active break event asserts a low logic level on the Break input to indicate an internal break event to external devices. Note: This bit cannot be modified as long as LOCK level 1 has been programmed (LOCK bits in TIM15_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
pub type BKBID_R = crate::BitReader<bool>;
#[doc = "Field `BKBID` writer - Break bidirectional In the bidirectional mode (BKBID bit set to 1), the break input is configured both in input mode and in open drain output mode. Any active break event asserts a low logic level on the Break input to indicate an internal break event to external devices. Note: This bit cannot be modified as long as LOCK level 1 has been programmed (LOCK bits in TIM15_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
pub type BKBID_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM15_BDTR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - Dead-time generator setup This bit-field defines the duration of the dead-time inserted between the complementary outputs. DT correspond to this duration. DTG\\[7:5\\]=0xx => DT=DTG\\[7:0\\]x tdtg with tdtg=tDTS DTG\\[7:5\\]=10x => DT=(64+DTG\\[5:0\\])xtdtg with Tdtg=2xtDTS DTG\\[7:5\\]=110 => DT=(32+DTG\\[4:0\\])xtdtg with Tdtg=8xtDTS DTG\\[7:5\\]=111 => DT=(32+DTG\\[4:0\\])xtdtg with Tdtg=16xtDTS Example if TDTS=125ns (8MHz), dead-time possible values are: 0 to 15875 ns by 125 ns steps, 16 �s to 31750 ns by 250 ns steps, 32 �s to 63 �s by 1 �s steps, 64 �s to 126 �s by 2 �s steps Note: This bit-field can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIM15_BDTR register)."]
    #[inline(always)]
    pub fn dtg(&self) -> DTG_R {
        DTG_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - Lock configuration These bits offer a write protection against software errors. Note: The LOCK bits can be written only once after the reset. Once the TIM15_BDTR register has been written, their content is frozen until the next reset."]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Off-state selection for Idle mode This bit is used when MOE=0 on channels configured as outputs. See tim_ocx/tim_ocxn enable description for more details (capture/compare enable register (TIM15_CCER) on page 1985). Note: This bit can not be modified as soon as the LOCK level 2 has been programmed (LOCK bits in TIM15_BDTR register)."]
    #[inline(always)]
    pub fn ossi(&self) -> OSSI_R {
        OSSI_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Off-state selection for Run mode This bit is used when MOE=1 on channels that have a complementary output which are configured as outputs. OSSR is not implemented if no complementary output is implemented in the timer. See tim_ocx/tim_ocxn enable description for more details (capture/compare enable register (TIM15_CCER) on page 1985). Note: This bit can not be modified as soon as the LOCK level 2 has been programmed (LOCK bits in TIM15_BDTR register)."]
    #[inline(always)]
    pub fn ossr(&self) -> OSSR_R {
        OSSR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Break enable 1; Break inputs (tim_brk and tim_sys_brk clock failure event) enabled This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIM15_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
    #[inline(always)]
    pub fn bke(&self) -> BKE_R {
        BKE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Break polarity Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIM15_BDTR register). Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
    #[inline(always)]
    pub fn bkp(&self) -> BKP_R {
        BKP_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Automatic output enable Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIM15_BDTR register)."]
    #[inline(always)]
    pub fn aoe(&self) -> AOE_R {
        AOE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Main output enable This bit is cleared asynchronously by hardware as soon as the tim_brk input is active. It is set by software or automatically depending on the AOE bit. It is acting only on the channels which are configured in output. See tim_ocx/tim_ocxn enable description for more details (capture/compare enable register (TIM15_CCER) on page 1985)."]
    #[inline(always)]
    pub fn moe(&self) -> MOE_R {
        MOE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Break filter This bit-field defines the frequency used to sample the tim_brk input signal and the length of the digital filter applied to tim_brk. The digital filter is made of an event counter in which N events are needed to validate a transition on the output: Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIM15_BDTR register)."]
    #[inline(always)]
    pub fn bkf(&self) -> BKF_R {
        BKF_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 26 - Break disarm This bit is cleared by hardware when no break source is active. The BKDSRM bit must be set by software to release the bidirectional output control (open-drain output in Hi-Z state) and then be polled until it is reset by hardware, indicating that the fault condition has disappeared. Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
    #[inline(always)]
    pub fn bkdsrm(&self) -> BKDSRM_R {
        BKDSRM_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Break bidirectional In the bidirectional mode (BKBID bit set to 1), the break input is configured both in input mode and in open drain output mode. Any active break event asserts a low logic level on the Break input to indicate an internal break event to external devices. Note: This bit cannot be modified as long as LOCK level 1 has been programmed (LOCK bits in TIM15_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
    #[inline(always)]
    pub fn bkbid(&self) -> BKBID_R {
        BKBID_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Dead-time generator setup This bit-field defines the duration of the dead-time inserted between the complementary outputs. DT correspond to this duration. DTG\\[7:5\\]=0xx => DT=DTG\\[7:0\\]x tdtg with tdtg=tDTS DTG\\[7:5\\]=10x => DT=(64+DTG\\[5:0\\])xtdtg with Tdtg=2xtDTS DTG\\[7:5\\]=110 => DT=(32+DTG\\[4:0\\])xtdtg with Tdtg=8xtDTS DTG\\[7:5\\]=111 => DT=(32+DTG\\[4:0\\])xtdtg with Tdtg=16xtDTS Example if TDTS=125ns (8MHz), dead-time possible values are: 0 to 15875 ns by 125 ns steps, 16 �s to 31750 ns by 250 ns steps, 32 �s to 63 �s by 1 �s steps, 64 �s to 126 �s by 2 �s steps Note: This bit-field can not be modified as long as LOCK level 1, 2 or 3 has been programmed (LOCK bits in TIM15_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn dtg(&mut self) -> DTG_W<0> {
        DTG_W::new(self)
    }
    #[doc = "Bits 8:9 - Lock configuration These bits offer a write protection against software errors. Note: The LOCK bits can be written only once after the reset. Once the TIM15_BDTR register has been written, their content is frozen until the next reset."]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<8> {
        LOCK_W::new(self)
    }
    #[doc = "Bit 10 - Off-state selection for Idle mode This bit is used when MOE=0 on channels configured as outputs. See tim_ocx/tim_ocxn enable description for more details (capture/compare enable register (TIM15_CCER) on page 1985). Note: This bit can not be modified as soon as the LOCK level 2 has been programmed (LOCK bits in TIM15_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn ossi(&mut self) -> OSSI_W<10> {
        OSSI_W::new(self)
    }
    #[doc = "Bit 11 - Off-state selection for Run mode This bit is used when MOE=1 on channels that have a complementary output which are configured as outputs. OSSR is not implemented if no complementary output is implemented in the timer. See tim_ocx/tim_ocxn enable description for more details (capture/compare enable register (TIM15_CCER) on page 1985). Note: This bit can not be modified as soon as the LOCK level 2 has been programmed (LOCK bits in TIM15_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn ossr(&mut self) -> OSSR_W<11> {
        OSSR_W::new(self)
    }
    #[doc = "Bit 12 - Break enable 1; Break inputs (tim_brk and tim_sys_brk clock failure event) enabled This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIM15_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
    #[inline(always)]
    #[must_use]
    pub fn bke(&mut self) -> BKE_W<12> {
        BKE_W::new(self)
    }
    #[doc = "Bit 13 - Break polarity Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIM15_BDTR register). Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
    #[inline(always)]
    #[must_use]
    pub fn bkp(&mut self) -> BKP_W<13> {
        BKP_W::new(self)
    }
    #[doc = "Bit 14 - Automatic output enable Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIM15_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn aoe(&mut self) -> AOE_W<14> {
        AOE_W::new(self)
    }
    #[doc = "Bit 15 - Main output enable This bit is cleared asynchronously by hardware as soon as the tim_brk input is active. It is set by software or automatically depending on the AOE bit. It is acting only on the channels which are configured in output. See tim_ocx/tim_ocxn enable description for more details (capture/compare enable register (TIM15_CCER) on page 1985)."]
    #[inline(always)]
    #[must_use]
    pub fn moe(&mut self) -> MOE_W<15> {
        MOE_W::new(self)
    }
    #[doc = "Bits 16:19 - Break filter This bit-field defines the frequency used to sample the tim_brk input signal and the length of the digital filter applied to tim_brk. The digital filter is made of an event counter in which N events are needed to validate a transition on the output: Note: This bit cannot be modified when LOCK level 1 has been programmed (LOCK bits in TIM15_BDTR register)."]
    #[inline(always)]
    #[must_use]
    pub fn bkf(&mut self) -> BKF_W<16> {
        BKF_W::new(self)
    }
    #[doc = "Bit 26 - Break disarm This bit is cleared by hardware when no break source is active. The BKDSRM bit must be set by software to release the bidirectional output control (open-drain output in Hi-Z state) and then be polled until it is reset by hardware, indicating that the fault condition has disappeared. Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
    #[inline(always)]
    #[must_use]
    pub fn bkdsrm(&mut self) -> BKDSRM_W<26> {
        BKDSRM_W::new(self)
    }
    #[doc = "Bit 28 - Break bidirectional In the bidirectional mode (BKBID bit set to 1), the break input is configured both in input mode and in open drain output mode. Any active break event asserts a low logic level on the Break input to indicate an internal break event to external devices. Note: This bit cannot be modified as long as LOCK level 1 has been programmed (LOCK bits in TIM15_BDTR register). Note: Any write operation to this bit takes a delay of 1 APB clock cycle to become effective."]
    #[inline(always)]
    #[must_use]
    pub fn bkbid(&mut self) -> BKBID_W<28> {
        BKBID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM15 break and dead-time register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim15_bdtr](index.html) module"]
pub struct TIM15_BDTR_SPEC;
impl crate::RegisterSpec for TIM15_BDTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tim15_bdtr::R](R) reader structure"]
impl crate::Readable for TIM15_BDTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim15_bdtr::W](W) writer structure"]
impl crate::Writable for TIM15_BDTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIM15_BDTR to value 0"]
impl crate::Resettable for TIM15_BDTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
