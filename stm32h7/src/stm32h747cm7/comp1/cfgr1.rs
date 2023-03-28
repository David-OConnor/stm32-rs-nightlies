#[doc = "Register `CFGR1` reader"]
pub struct R(crate::R<CFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR1` writer"]
pub struct W(crate::W<CFGR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR1_SPEC>;
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
impl From<crate::W<CFGR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - COMP channel 1 enable bit"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - COMP channel 1 enable bit"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, bool, O>;
#[doc = "Field `BRGEN` reader - Scaler bridge enable"]
pub type BRGEN_R = crate::BitReader<bool>;
#[doc = "Field `BRGEN` writer - Scaler bridge enable"]
pub type BRGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, bool, O>;
#[doc = "Field `SCALEN` reader - Voltage scaler enable bit"]
pub type SCALEN_R = crate::BitReader<bool>;
#[doc = "Field `SCALEN` writer - Voltage scaler enable bit"]
pub type SCALEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, bool, O>;
#[doc = "Field `POLARITY` reader - COMP channel 1 polarity selection bit"]
pub type POLARITY_R = crate::BitReader<bool>;
#[doc = "Field `POLARITY` writer - COMP channel 1 polarity selection bit"]
pub type POLARITY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, bool, O>;
#[doc = "Field `ITEN` reader - COMP channel 1 interrupt enable"]
pub type ITEN_R = crate::BitReader<bool>;
#[doc = "Field `ITEN` writer - COMP channel 1 interrupt enable"]
pub type ITEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, bool, O>;
#[doc = "Field `HYST` reader - COMP channel 1 hysteresis selection bits"]
pub type HYST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HYST` writer - COMP channel 1 hysteresis selection bits"]
pub type HYST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `PWRMODE` reader - Power Mode of the COMP channel 1"]
pub type PWRMODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PWRMODE` writer - Power Mode of the COMP channel 1"]
pub type PWRMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `INMSEL` reader - COMP channel 1 inverting input selection field"]
pub type INMSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INMSEL` writer - COMP channel 1 inverting input selection field"]
pub type INMSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR1_SPEC, u8, u8, 3, O>;
#[doc = "Field `INPSEL` reader - COMP channel 1 non-inverting input selection bit"]
pub type INPSEL_R = crate::BitReader<bool>;
#[doc = "Field `INPSEL` writer - COMP channel 1 non-inverting input selection bit"]
pub type INPSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, bool, O>;
#[doc = "Field `BLANKING` reader - COMP channel 1 blanking source selection bits"]
pub type BLANKING_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BLANKING` writer - COMP channel 1 blanking source selection bits"]
pub type BLANKING_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `LOCK` reader - Lock bit"]
pub type LOCK_R = crate::BitReader<bool>;
#[doc = "Field `LOCK` writer - Lock bit"]
pub type LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - COMP channel 1 enable bit"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Scaler bridge enable"]
    #[inline(always)]
    pub fn brgen(&self) -> BRGEN_R {
        BRGEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Voltage scaler enable bit"]
    #[inline(always)]
    pub fn scalen(&self) -> SCALEN_R {
        SCALEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - COMP channel 1 polarity selection bit"]
    #[inline(always)]
    pub fn polarity(&self) -> POLARITY_R {
        POLARITY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - COMP channel 1 interrupt enable"]
    #[inline(always)]
    pub fn iten(&self) -> ITEN_R {
        ITEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:9 - COMP channel 1 hysteresis selection bits"]
    #[inline(always)]
    pub fn hyst(&self) -> HYST_R {
        HYST_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Power Mode of the COMP channel 1"]
    #[inline(always)]
    pub fn pwrmode(&self) -> PWRMODE_R {
        PWRMODE_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:18 - COMP channel 1 inverting input selection field"]
    #[inline(always)]
    pub fn inmsel(&self) -> INMSEL_R {
        INMSEL_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 20 - COMP channel 1 non-inverting input selection bit"]
    #[inline(always)]
    pub fn inpsel(&self) -> INPSEL_R {
        INPSEL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 24:27 - COMP channel 1 blanking source selection bits"]
    #[inline(always)]
    pub fn blanking(&self) -> BLANKING_R {
        BLANKING_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - Lock bit"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - COMP channel 1 enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - Scaler bridge enable"]
    #[inline(always)]
    #[must_use]
    pub fn brgen(&mut self) -> BRGEN_W<1> {
        BRGEN_W::new(self)
    }
    #[doc = "Bit 2 - Voltage scaler enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn scalen(&mut self) -> SCALEN_W<2> {
        SCALEN_W::new(self)
    }
    #[doc = "Bit 3 - COMP channel 1 polarity selection bit"]
    #[inline(always)]
    #[must_use]
    pub fn polarity(&mut self) -> POLARITY_W<3> {
        POLARITY_W::new(self)
    }
    #[doc = "Bit 6 - COMP channel 1 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn iten(&mut self) -> ITEN_W<6> {
        ITEN_W::new(self)
    }
    #[doc = "Bits 8:9 - COMP channel 1 hysteresis selection bits"]
    #[inline(always)]
    #[must_use]
    pub fn hyst(&mut self) -> HYST_W<8> {
        HYST_W::new(self)
    }
    #[doc = "Bits 12:13 - Power Mode of the COMP channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn pwrmode(&mut self) -> PWRMODE_W<12> {
        PWRMODE_W::new(self)
    }
    #[doc = "Bits 16:18 - COMP channel 1 inverting input selection field"]
    #[inline(always)]
    #[must_use]
    pub fn inmsel(&mut self) -> INMSEL_W<16> {
        INMSEL_W::new(self)
    }
    #[doc = "Bit 20 - COMP channel 1 non-inverting input selection bit"]
    #[inline(always)]
    #[must_use]
    pub fn inpsel(&mut self) -> INPSEL_W<20> {
        INPSEL_W::new(self)
    }
    #[doc = "Bits 24:27 - COMP channel 1 blanking source selection bits"]
    #[inline(always)]
    #[must_use]
    pub fn blanking(&mut self) -> BLANKING_W<24> {
        BLANKING_W::new(self)
    }
    #[doc = "Bit 31 - Lock bit"]
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
#[doc = "Comparator configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr1](index.html) module"]
pub struct CFGR1_SPEC;
impl crate::RegisterSpec for CFGR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr1::R](R) reader structure"]
impl crate::Readable for CFGR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr1::W](W) writer structure"]
impl crate::Writable for CFGR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFGR1 to value 0"]
impl crate::Resettable for CFGR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
