#[doc = "Register `COMP1_CSR` reader"]
pub struct R(crate::R<COMP1_CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP1_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP1_CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP1_CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMP1_CSR` writer"]
pub struct W(crate::W<COMP1_CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMP1_CSR_SPEC>;
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
impl From<crate::W<COMP1_CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMP1_CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMP1_EN` reader - Comparator 1 enable bit"]
pub type COMP1_EN_R = crate::BitReader<bool>;
#[doc = "Field `COMP1_EN` writer - Comparator 1 enable bit"]
pub type COMP1_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP1_CSR_SPEC, bool, O>;
#[doc = "Field `COMP1_PWRMODE` reader - Power Mode of the comparator 1"]
pub type COMP1_PWRMODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMP1_PWRMODE` writer - Power Mode of the comparator 1"]
pub type COMP1_PWRMODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COMP1_CSR_SPEC, u8, u8, 2, O>;
#[doc = "Field `COMP1_INMSEL` reader - Comparator 1 Input Minus connection configuration bit"]
pub type COMP1_INMSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMP1_INMSEL` writer - Comparator 1 Input Minus connection configuration bit"]
pub type COMP1_INMSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COMP1_CSR_SPEC, u8, u8, 3, O>;
#[doc = "Field `COMP1_INPSEL` reader - Comparator1 input plus selection bit"]
pub type COMP1_INPSEL_R = crate::BitReader<bool>;
#[doc = "Field `COMP1_INPSEL` writer - Comparator1 input plus selection bit"]
pub type COMP1_INPSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP1_CSR_SPEC, bool, O>;
#[doc = "Field `COMP1_POLARITY` reader - Comparator 1 polarity selection bit"]
pub type COMP1_POLARITY_R = crate::BitReader<bool>;
#[doc = "Field `COMP1_POLARITY` writer - Comparator 1 polarity selection bit"]
pub type COMP1_POLARITY_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP1_CSR_SPEC, bool, O>;
#[doc = "Field `COMP1_HYST` reader - Comparator 1 hysteresis selection bits"]
pub type COMP1_HYST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMP1_HYST` writer - Comparator 1 hysteresis selection bits"]
pub type COMP1_HYST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COMP1_CSR_SPEC, u8, u8, 2, O>;
#[doc = "Field `COMP1_BLANKING` reader - Comparator 1 blanking source selection bits"]
pub type COMP1_BLANKING_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMP1_BLANKING` writer - Comparator 1 blanking source selection bits"]
pub type COMP1_BLANKING_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COMP1_CSR_SPEC, u8, u8, 3, O>;
#[doc = "Field `COMP1_BRGEN` reader - Scaler bridge enable"]
pub type COMP1_BRGEN_R = crate::BitReader<bool>;
#[doc = "Field `COMP1_BRGEN` writer - Scaler bridge enable"]
pub type COMP1_BRGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP1_CSR_SPEC, bool, O>;
#[doc = "Field `COMP1_SCALEN` reader - Voltage scaler enable bit"]
pub type COMP1_SCALEN_R = crate::BitReader<bool>;
#[doc = "Field `COMP1_SCALEN` writer - Voltage scaler enable bit"]
pub type COMP1_SCALEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP1_CSR_SPEC, bool, O>;
#[doc = "Field `COMP1_VALUE` reader - Comparator 1 output status bit"]
pub type COMP1_VALUE_R = crate::BitReader<bool>;
#[doc = "Field `COMP1_LOCK` writer - COMP1_CSR register lock bit"]
pub type COMP1_LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP1_CSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Comparator 1 enable bit"]
    #[inline(always)]
    pub fn comp1_en(&self) -> COMP1_EN_R {
        COMP1_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:3 - Power Mode of the comparator 1"]
    #[inline(always)]
    pub fn comp1_pwrmode(&self) -> COMP1_PWRMODE_R {
        COMP1_PWRMODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - Comparator 1 Input Minus connection configuration bit"]
    #[inline(always)]
    pub fn comp1_inmsel(&self) -> COMP1_INMSEL_R {
        COMP1_INMSEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Comparator1 input plus selection bit"]
    #[inline(always)]
    pub fn comp1_inpsel(&self) -> COMP1_INPSEL_R {
        COMP1_INPSEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 15 - Comparator 1 polarity selection bit"]
    #[inline(always)]
    pub fn comp1_polarity(&self) -> COMP1_POLARITY_R {
        COMP1_POLARITY_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Comparator 1 hysteresis selection bits"]
    #[inline(always)]
    pub fn comp1_hyst(&self) -> COMP1_HYST_R {
        COMP1_HYST_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:20 - Comparator 1 blanking source selection bits"]
    #[inline(always)]
    pub fn comp1_blanking(&self) -> COMP1_BLANKING_R {
        COMP1_BLANKING_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 22 - Scaler bridge enable"]
    #[inline(always)]
    pub fn comp1_brgen(&self) -> COMP1_BRGEN_R {
        COMP1_BRGEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Voltage scaler enable bit"]
    #[inline(always)]
    pub fn comp1_scalen(&self) -> COMP1_SCALEN_R {
        COMP1_SCALEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 30 - Comparator 1 output status bit"]
    #[inline(always)]
    pub fn comp1_value(&self) -> COMP1_VALUE_R {
        COMP1_VALUE_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 1 enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn comp1_en(&mut self) -> COMP1_EN_W<0> {
        COMP1_EN_W::new(self)
    }
    #[doc = "Bits 2:3 - Power Mode of the comparator 1"]
    #[inline(always)]
    #[must_use]
    pub fn comp1_pwrmode(&mut self) -> COMP1_PWRMODE_W<2> {
        COMP1_PWRMODE_W::new(self)
    }
    #[doc = "Bits 4:6 - Comparator 1 Input Minus connection configuration bit"]
    #[inline(always)]
    #[must_use]
    pub fn comp1_inmsel(&mut self) -> COMP1_INMSEL_W<4> {
        COMP1_INMSEL_W::new(self)
    }
    #[doc = "Bit 7 - Comparator1 input plus selection bit"]
    #[inline(always)]
    #[must_use]
    pub fn comp1_inpsel(&mut self) -> COMP1_INPSEL_W<7> {
        COMP1_INPSEL_W::new(self)
    }
    #[doc = "Bit 15 - Comparator 1 polarity selection bit"]
    #[inline(always)]
    #[must_use]
    pub fn comp1_polarity(&mut self) -> COMP1_POLARITY_W<15> {
        COMP1_POLARITY_W::new(self)
    }
    #[doc = "Bits 16:17 - Comparator 1 hysteresis selection bits"]
    #[inline(always)]
    #[must_use]
    pub fn comp1_hyst(&mut self) -> COMP1_HYST_W<16> {
        COMP1_HYST_W::new(self)
    }
    #[doc = "Bits 18:20 - Comparator 1 blanking source selection bits"]
    #[inline(always)]
    #[must_use]
    pub fn comp1_blanking(&mut self) -> COMP1_BLANKING_W<18> {
        COMP1_BLANKING_W::new(self)
    }
    #[doc = "Bit 22 - Scaler bridge enable"]
    #[inline(always)]
    #[must_use]
    pub fn comp1_brgen(&mut self) -> COMP1_BRGEN_W<22> {
        COMP1_BRGEN_W::new(self)
    }
    #[doc = "Bit 23 - Voltage scaler enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn comp1_scalen(&mut self) -> COMP1_SCALEN_W<23> {
        COMP1_SCALEN_W::new(self)
    }
    #[doc = "Bit 31 - COMP1_CSR register lock bit"]
    #[inline(always)]
    #[must_use]
    pub fn comp1_lock(&mut self) -> COMP1_LOCK_W<31> {
        COMP1_LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Comparator 1 control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp1_csr](index.html) module"]
pub struct COMP1_CSR_SPEC;
impl crate::RegisterSpec for COMP1_CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comp1_csr::R](R) reader structure"]
impl crate::Readable for COMP1_CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [comp1_csr::W](W) writer structure"]
impl crate::Writable for COMP1_CSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COMP1_CSR to value 0"]
impl crate::Resettable for COMP1_CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
