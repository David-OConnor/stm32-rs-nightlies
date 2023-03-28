#[doc = "Register `COMP_CFGR1` reader"]
pub struct R(crate::R<COMP_CFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP_CFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP_CFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP_CFGR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMP_CFGR1` writer"]
pub struct W(crate::W<COMP_CFGR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMP_CFGR1_SPEC>;
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
impl From<crate::W<COMP_CFGR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMP_CFGR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - COMP Channel1 enable This bit is set and cleared by software (only if LOCK not set). It enables the COMP�Channel1."]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - COMP Channel1 enable This bit is set and cleared by software (only if LOCK not set). It enables the COMP�Channel1."]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP_CFGR1_SPEC, bool, O>;
#[doc = "Field `BRGEN` reader - Scaler bridge enable This bit is set and cleared by software (only if LOCK not set). This bit enables the bridge of the scaler. If SCALEN is set and BRGEN is reset, all four scaler outputs provide the same level V<sub>REF_COMP</sub> (similar to V<sub>REFINT</sub>). If SCALEN and BRGEN are set, the four scaler outputs provide V<sub>REF_COMP</sub>, 3/4�V<sub>REF_COMP</sub>, 1/2�V<sub>REF_COMP</sub> and 1/4�V<sub>REF_COMP</sub> levels, respectively."]
pub type BRGEN_R = crate::BitReader<bool>;
#[doc = "Field `BRGEN` writer - Scaler bridge enable This bit is set and cleared by software (only if LOCK not set). This bit enables the bridge of the scaler. If SCALEN is set and BRGEN is reset, all four scaler outputs provide the same level V<sub>REF_COMP</sub> (similar to V<sub>REFINT</sub>). If SCALEN and BRGEN are set, the four scaler outputs provide V<sub>REF_COMP</sub>, 3/4�V<sub>REF_COMP</sub>, 1/2�V<sub>REF_COMP</sub> and 1/4�V<sub>REF_COMP</sub> levels, respectively."]
pub type BRGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP_CFGR1_SPEC, bool, O>;
#[doc = "Field `SCALEN` reader - Voltage scaler enable This bit is set and cleared by software (only if LOCK not set). This bit enables the V<sub>REFINT</sub> scaler for the COMP channels."]
pub type SCALEN_R = crate::BitReader<bool>;
#[doc = "Field `SCALEN` writer - Voltage scaler enable This bit is set and cleared by software (only if LOCK not set). This bit enables the V<sub>REFINT</sub> scaler for the COMP channels."]
pub type SCALEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP_CFGR1_SPEC, bool, O>;
#[doc = "Field `POLARITY` reader - COMP channel1 polarity selection This bit is set and cleared by software (only if LOCK not set). It inverts COMP channel1 polarity."]
pub type POLARITY_R = crate::BitReader<bool>;
#[doc = "Field `POLARITY` writer - COMP channel1 polarity selection This bit is set and cleared by software (only if LOCK not set). It inverts COMP channel1 polarity."]
pub type POLARITY_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP_CFGR1_SPEC, bool, O>;
#[doc = "Field `ITEN` reader - COMP channel1 interrupt enable This bit is set and cleared by software (only if LOCK not set). This bit enable the interrupt generation of the COMP channel1."]
pub type ITEN_R = crate::BitReader<bool>;
#[doc = "Field `ITEN` writer - COMP channel1 interrupt enable This bit is set and cleared by software (only if LOCK not set). This bit enable the interrupt generation of the COMP channel1."]
pub type ITEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP_CFGR1_SPEC, bool, O>;
#[doc = "Field `HYST` reader - COMP channel1 hysteresis selection These bits are set and cleared by software (only if LOCK not set). They select the hysteresis voltage of the COMP channel1."]
pub type HYST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HYST` writer - COMP channel1 hysteresis selection These bits are set and cleared by software (only if LOCK not set). They select the hysteresis voltage of the COMP channel1."]
pub type HYST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COMP_CFGR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `PWRMODE` reader - Power mode of the COMP channel1 These bits are set and cleared by software (only if LOCK not set). They control the power/speed of the COMP channel1."]
pub type PWRMODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PWRMODE` writer - Power mode of the COMP channel1 These bits are set and cleared by software (only if LOCK not set). They control the power/speed of the COMP channel1."]
pub type PWRMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COMP_CFGR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `INMSEL` reader - COMP channel1 inverting input selection These bits are set and cleared by software (only if LOCK not set). They select which input is connected to the input minus of the COMP channel. Note: See Table�146: COMP1 inverting input assignment for more details."]
pub type INMSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INMSEL` writer - COMP channel1 inverting input selection These bits are set and cleared by software (only if LOCK not set). They select which input is connected to the input minus of the COMP channel. Note: See Table�146: COMP1 inverting input assignment for more details."]
pub type INMSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COMP_CFGR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `INPSEL1` reader - COMP noninverting input selection This bit is set and cleared by software (only if LOCK not set). They select which input is connected to the positive input of COMP channel. Note: See Table�145: COMP1 noninverting input assignment for more details."]
pub type INPSEL1_R = crate::BitReader<bool>;
#[doc = "Field `INPSEL1` writer - COMP noninverting input selection This bit is set and cleared by software (only if LOCK not set). They select which input is connected to the positive input of COMP channel. Note: See Table�145: COMP1 noninverting input assignment for more details."]
pub type INPSEL1_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP_CFGR1_SPEC, bool, O>;
#[doc = "Field `INPSEL2` reader - COMP noninverting input selection This bit is set and cleared by software (only if LOCK not set). They select which input is connected to the positive input of the COMP channel. See Table�145: COMP1 noninverting input assignment for more details."]
pub type INPSEL2_R = crate::BitReader<bool>;
#[doc = "Field `INPSEL2` writer - COMP noninverting input selection This bit is set and cleared by software (only if LOCK not set). They select which input is connected to the positive input of the COMP channel. See Table�145: COMP1 noninverting input assignment for more details."]
pub type INPSEL2_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP_CFGR1_SPEC, bool, O>;
#[doc = "Field `BLANKING` reader - COMP Channel1 blanking source selection Bits of this field are set and cleared by software (only if LOCK not set). The field selects the input source for COMP Channel1 output blanking: All other values: reserved"]
pub type BLANKING_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BLANKING` writer - COMP Channel1 blanking source selection Bits of this field are set and cleared by software (only if LOCK not set). The field selects the input source for COMP Channel1 output blanking: All other values: reserved"]
pub type BLANKING_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COMP_CFGR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `LOCK` reader - Lock This bit is set by software and cleared by a hardware system reset. It locks the whole content of the COMP Channel1 configuration register COMP_CFGR1\\[31:0\\]"]
pub type LOCK_R = crate::BitReader<bool>;
#[doc = "Field `LOCK` writer - Lock This bit is set by software and cleared by a hardware system reset. It locks the whole content of the COMP Channel1 configuration register COMP_CFGR1\\[31:0\\]"]
pub type LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP_CFGR1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - COMP Channel1 enable This bit is set and cleared by software (only if LOCK not set). It enables the COMP�Channel1."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Scaler bridge enable This bit is set and cleared by software (only if LOCK not set). This bit enables the bridge of the scaler. If SCALEN is set and BRGEN is reset, all four scaler outputs provide the same level V<sub>REF_COMP</sub> (similar to V<sub>REFINT</sub>). If SCALEN and BRGEN are set, the four scaler outputs provide V<sub>REF_COMP</sub>, 3/4�V<sub>REF_COMP</sub>, 1/2�V<sub>REF_COMP</sub> and 1/4�V<sub>REF_COMP</sub> levels, respectively."]
    #[inline(always)]
    pub fn brgen(&self) -> BRGEN_R {
        BRGEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Voltage scaler enable This bit is set and cleared by software (only if LOCK not set). This bit enables the V<sub>REFINT</sub> scaler for the COMP channels."]
    #[inline(always)]
    pub fn scalen(&self) -> SCALEN_R {
        SCALEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - COMP channel1 polarity selection This bit is set and cleared by software (only if LOCK not set). It inverts COMP channel1 polarity."]
    #[inline(always)]
    pub fn polarity(&self) -> POLARITY_R {
        POLARITY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - COMP channel1 interrupt enable This bit is set and cleared by software (only if LOCK not set). This bit enable the interrupt generation of the COMP channel1."]
    #[inline(always)]
    pub fn iten(&self) -> ITEN_R {
        ITEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:9 - COMP channel1 hysteresis selection These bits are set and cleared by software (only if LOCK not set). They select the hysteresis voltage of the COMP channel1."]
    #[inline(always)]
    pub fn hyst(&self) -> HYST_R {
        HYST_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Power mode of the COMP channel1 These bits are set and cleared by software (only if LOCK not set). They control the power/speed of the COMP channel1."]
    #[inline(always)]
    pub fn pwrmode(&self) -> PWRMODE_R {
        PWRMODE_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:19 - COMP channel1 inverting input selection These bits are set and cleared by software (only if LOCK not set). They select which input is connected to the input minus of the COMP channel. Note: See Table�146: COMP1 inverting input assignment for more details."]
    #[inline(always)]
    pub fn inmsel(&self) -> INMSEL_R {
        INMSEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - COMP noninverting input selection This bit is set and cleared by software (only if LOCK not set). They select which input is connected to the positive input of COMP channel. Note: See Table�145: COMP1 noninverting input assignment for more details."]
    #[inline(always)]
    pub fn inpsel1(&self) -> INPSEL1_R {
        INPSEL1_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - COMP noninverting input selection This bit is set and cleared by software (only if LOCK not set). They select which input is connected to the positive input of the COMP channel. See Table�145: COMP1 noninverting input assignment for more details."]
    #[inline(always)]
    pub fn inpsel2(&self) -> INPSEL2_R {
        INPSEL2_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 24:27 - COMP Channel1 blanking source selection Bits of this field are set and cleared by software (only if LOCK not set). The field selects the input source for COMP Channel1 output blanking: All other values: reserved"]
    #[inline(always)]
    pub fn blanking(&self) -> BLANKING_R {
        BLANKING_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - Lock This bit is set by software and cleared by a hardware system reset. It locks the whole content of the COMP Channel1 configuration register COMP_CFGR1\\[31:0\\]"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - COMP Channel1 enable This bit is set and cleared by software (only if LOCK not set). It enables the COMP�Channel1."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - Scaler bridge enable This bit is set and cleared by software (only if LOCK not set). This bit enables the bridge of the scaler. If SCALEN is set and BRGEN is reset, all four scaler outputs provide the same level V<sub>REF_COMP</sub> (similar to V<sub>REFINT</sub>). If SCALEN and BRGEN are set, the four scaler outputs provide V<sub>REF_COMP</sub>, 3/4�V<sub>REF_COMP</sub>, 1/2�V<sub>REF_COMP</sub> and 1/4�V<sub>REF_COMP</sub> levels, respectively."]
    #[inline(always)]
    #[must_use]
    pub fn brgen(&mut self) -> BRGEN_W<1> {
        BRGEN_W::new(self)
    }
    #[doc = "Bit 2 - Voltage scaler enable This bit is set and cleared by software (only if LOCK not set). This bit enables the V<sub>REFINT</sub> scaler for the COMP channels."]
    #[inline(always)]
    #[must_use]
    pub fn scalen(&mut self) -> SCALEN_W<2> {
        SCALEN_W::new(self)
    }
    #[doc = "Bit 3 - COMP channel1 polarity selection This bit is set and cleared by software (only if LOCK not set). It inverts COMP channel1 polarity."]
    #[inline(always)]
    #[must_use]
    pub fn polarity(&mut self) -> POLARITY_W<3> {
        POLARITY_W::new(self)
    }
    #[doc = "Bit 6 - COMP channel1 interrupt enable This bit is set and cleared by software (only if LOCK not set). This bit enable the interrupt generation of the COMP channel1."]
    #[inline(always)]
    #[must_use]
    pub fn iten(&mut self) -> ITEN_W<6> {
        ITEN_W::new(self)
    }
    #[doc = "Bits 8:9 - COMP channel1 hysteresis selection These bits are set and cleared by software (only if LOCK not set). They select the hysteresis voltage of the COMP channel1."]
    #[inline(always)]
    #[must_use]
    pub fn hyst(&mut self) -> HYST_W<8> {
        HYST_W::new(self)
    }
    #[doc = "Bits 12:13 - Power mode of the COMP channel1 These bits are set and cleared by software (only if LOCK not set). They control the power/speed of the COMP channel1."]
    #[inline(always)]
    #[must_use]
    pub fn pwrmode(&mut self) -> PWRMODE_W<12> {
        PWRMODE_W::new(self)
    }
    #[doc = "Bits 16:19 - COMP channel1 inverting input selection These bits are set and cleared by software (only if LOCK not set). They select which input is connected to the input minus of the COMP channel. Note: See Table�146: COMP1 inverting input assignment for more details."]
    #[inline(always)]
    #[must_use]
    pub fn inmsel(&mut self) -> INMSEL_W<16> {
        INMSEL_W::new(self)
    }
    #[doc = "Bit 20 - COMP noninverting input selection This bit is set and cleared by software (only if LOCK not set). They select which input is connected to the positive input of COMP channel. Note: See Table�145: COMP1 noninverting input assignment for more details."]
    #[inline(always)]
    #[must_use]
    pub fn inpsel1(&mut self) -> INPSEL1_W<20> {
        INPSEL1_W::new(self)
    }
    #[doc = "Bit 22 - COMP noninverting input selection This bit is set and cleared by software (only if LOCK not set). They select which input is connected to the positive input of the COMP channel. See Table�145: COMP1 noninverting input assignment for more details."]
    #[inline(always)]
    #[must_use]
    pub fn inpsel2(&mut self) -> INPSEL2_W<22> {
        INPSEL2_W::new(self)
    }
    #[doc = "Bits 24:27 - COMP Channel1 blanking source selection Bits of this field are set and cleared by software (only if LOCK not set). The field selects the input source for COMP Channel1 output blanking: All other values: reserved"]
    #[inline(always)]
    #[must_use]
    pub fn blanking(&mut self) -> BLANKING_W<24> {
        BLANKING_W::new(self)
    }
    #[doc = "Bit 31 - Lock This bit is set by software and cleared by a hardware system reset. It locks the whole content of the COMP Channel1 configuration register COMP_CFGR1\\[31:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<31> {
        LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Comparator configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp_cfgr1](index.html) module"]
pub struct COMP_CFGR1_SPEC;
impl crate::RegisterSpec for COMP_CFGR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comp_cfgr1::R](R) reader structure"]
impl crate::Readable for COMP_CFGR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [comp_cfgr1::W](W) writer structure"]
impl crate::Writable for COMP_CFGR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COMP_CFGR1 to value 0"]
impl crate::Resettable for COMP_CFGR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
