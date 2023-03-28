#[doc = "Register `CSR` reader"]
pub struct R(crate::R<CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSR` writer"]
pub struct W(crate::W<CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR_SPEC>;
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
impl From<crate::W<CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMP1EN` reader - Comparator 1 enable"]
pub type COMP1EN_R = crate::BitReader<bool>;
#[doc = "Field `COMP1EN` writer - Comparator 1 enable"]
pub type COMP1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `COMP1_INP_DAC` reader - Comparator 1 non inverting input connection to DAC output"]
pub type COMP1_INP_DAC_R = crate::BitReader<bool>;
#[doc = "Field `COMP1_INP_DAC` writer - Comparator 1 non inverting input connection to DAC output"]
pub type COMP1_INP_DAC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `COMP1MODE` reader - Comparator 1 mode"]
pub type COMP1MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMP1MODE` writer - Comparator 1 mode"]
pub type COMP1MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSR_SPEC, u8, u8, 2, O>;
#[doc = "Field `COMP1INSEL` reader - Comparator 1 inverting input selection"]
pub type COMP1INSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMP1INSEL` writer - Comparator 1 inverting input selection"]
pub type COMP1INSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSR_SPEC, u8, u8, 3, O>;
#[doc = "Field `COMP1OUTSEL` reader - Comparator 1 output selection"]
pub type COMP1OUTSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMP1OUTSEL` writer - Comparator 1 output selection"]
pub type COMP1OUTSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSR_SPEC, u8, u8, 3, O>;
#[doc = "Field `COMP1POL` reader - Comparator 1 output polarity"]
pub type COMP1POL_R = crate::BitReader<bool>;
#[doc = "Field `COMP1POL` writer - Comparator 1 output polarity"]
pub type COMP1POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `COMP1HYST` reader - Comparator 1 hysteresis"]
pub type COMP1HYST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMP1HYST` writer - Comparator 1 hysteresis"]
pub type COMP1HYST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSR_SPEC, u8, u8, 2, O>;
#[doc = "Field `COMP1OUT` reader - Comparator 1 output"]
pub type COMP1OUT_R = crate::BitReader<bool>;
#[doc = "Field `COMP1LOCK` reader - Comparator 1 lock"]
pub type COMP1LOCK_R = crate::BitReader<bool>;
#[doc = "Field `COMP1LOCK` writer - Comparator 1 lock"]
pub type COMP1LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `COMP2EN` reader - Comparator 2 enable"]
pub type COMP2EN_R = crate::BitReader<bool>;
#[doc = "Field `COMP2EN` writer - Comparator 2 enable"]
pub type COMP2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `COMP2MODE` reader - Comparator 2 mode"]
pub type COMP2MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMP2MODE` writer - Comparator 2 mode"]
pub type COMP2MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSR_SPEC, u8, u8, 2, O>;
#[doc = "Field `COMP2INSEL` reader - Comparator 2 inverting input selection"]
pub type COMP2INSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMP2INSEL` writer - Comparator 2 inverting input selection"]
pub type COMP2INSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSR_SPEC, u8, u8, 3, O>;
#[doc = "Field `WNDWEN` reader - Window mode enable"]
pub type WNDWEN_R = crate::BitReader<bool>;
#[doc = "Field `WNDWEN` writer - Window mode enable"]
pub type WNDWEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `COMP2OUTSEL` reader - Comparator 2 output selection"]
pub type COMP2OUTSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMP2OUTSEL` writer - Comparator 2 output selection"]
pub type COMP2OUTSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSR_SPEC, u8, u8, 3, O>;
#[doc = "Field `COMP2POL` reader - Comparator 2 output polarity"]
pub type COMP2POL_R = crate::BitReader<bool>;
#[doc = "Field `COMP2POL` writer - Comparator 2 output polarity"]
pub type COMP2POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `COMP2HYST` reader - Comparator 2 hysteresis"]
pub type COMP2HYST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMP2HYST` writer - Comparator 2 hysteresis"]
pub type COMP2HYST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSR_SPEC, u8, u8, 2, O>;
#[doc = "Field `COMP2OUT` reader - Comparator 2 output"]
pub type COMP2OUT_R = crate::BitReader<bool>;
#[doc = "Field `COMP2LOCK` reader - Comparator 2 lock"]
pub type COMP2LOCK_R = crate::BitReader<bool>;
#[doc = "Field `COMP2LOCK` writer - Comparator 2 lock"]
pub type COMP2LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Comparator 1 enable"]
    #[inline(always)]
    pub fn comp1en(&self) -> COMP1EN_R {
        COMP1EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparator 1 non inverting input connection to DAC output"]
    #[inline(always)]
    pub fn comp1_inp_dac(&self) -> COMP1_INP_DAC_R {
        COMP1_INP_DAC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Comparator 1 mode"]
    #[inline(always)]
    pub fn comp1mode(&self) -> COMP1MODE_R {
        COMP1MODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - Comparator 1 inverting input selection"]
    #[inline(always)]
    pub fn comp1insel(&self) -> COMP1INSEL_R {
        COMP1INSEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Comparator 1 output selection"]
    #[inline(always)]
    pub fn comp1outsel(&self) -> COMP1OUTSEL_R {
        COMP1OUTSEL_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Comparator 1 output polarity"]
    #[inline(always)]
    pub fn comp1pol(&self) -> COMP1POL_R {
        COMP1POL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Comparator 1 hysteresis"]
    #[inline(always)]
    pub fn comp1hyst(&self) -> COMP1HYST_R {
        COMP1HYST_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Comparator 1 output"]
    #[inline(always)]
    pub fn comp1out(&self) -> COMP1OUT_R {
        COMP1OUT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Comparator 1 lock"]
    #[inline(always)]
    pub fn comp1lock(&self) -> COMP1LOCK_R {
        COMP1LOCK_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Comparator 2 enable"]
    #[inline(always)]
    pub fn comp2en(&self) -> COMP2EN_R {
        COMP2EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Comparator 2 mode"]
    #[inline(always)]
    pub fn comp2mode(&self) -> COMP2MODE_R {
        COMP2MODE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:22 - Comparator 2 inverting input selection"]
    #[inline(always)]
    pub fn comp2insel(&self) -> COMP2INSEL_R {
        COMP2INSEL_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - Window mode enable"]
    #[inline(always)]
    pub fn wndwen(&self) -> WNDWEN_R {
        WNDWEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Comparator 2 output selection"]
    #[inline(always)]
    pub fn comp2outsel(&self) -> COMP2OUTSEL_R {
        COMP2OUTSEL_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - Comparator 2 output polarity"]
    #[inline(always)]
    pub fn comp2pol(&self) -> COMP2POL_R {
        COMP2POL_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:29 - Comparator 2 hysteresis"]
    #[inline(always)]
    pub fn comp2hyst(&self) -> COMP2HYST_R {
        COMP2HYST_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - Comparator 2 output"]
    #[inline(always)]
    pub fn comp2out(&self) -> COMP2OUT_R {
        COMP2OUT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Comparator 2 lock"]
    #[inline(always)]
    pub fn comp2lock(&self) -> COMP2LOCK_R {
        COMP2LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn comp1en(&mut self) -> COMP1EN_W<0> {
        COMP1EN_W::new(self)
    }
    #[doc = "Bit 1 - Comparator 1 non inverting input connection to DAC output"]
    #[inline(always)]
    #[must_use]
    pub fn comp1_inp_dac(&mut self) -> COMP1_INP_DAC_W<1> {
        COMP1_INP_DAC_W::new(self)
    }
    #[doc = "Bits 2:3 - Comparator 1 mode"]
    #[inline(always)]
    #[must_use]
    pub fn comp1mode(&mut self) -> COMP1MODE_W<2> {
        COMP1MODE_W::new(self)
    }
    #[doc = "Bits 4:6 - Comparator 1 inverting input selection"]
    #[inline(always)]
    #[must_use]
    pub fn comp1insel(&mut self) -> COMP1INSEL_W<4> {
        COMP1INSEL_W::new(self)
    }
    #[doc = "Bits 8:10 - Comparator 1 output selection"]
    #[inline(always)]
    #[must_use]
    pub fn comp1outsel(&mut self) -> COMP1OUTSEL_W<8> {
        COMP1OUTSEL_W::new(self)
    }
    #[doc = "Bit 11 - Comparator 1 output polarity"]
    #[inline(always)]
    #[must_use]
    pub fn comp1pol(&mut self) -> COMP1POL_W<11> {
        COMP1POL_W::new(self)
    }
    #[doc = "Bits 12:13 - Comparator 1 hysteresis"]
    #[inline(always)]
    #[must_use]
    pub fn comp1hyst(&mut self) -> COMP1HYST_W<12> {
        COMP1HYST_W::new(self)
    }
    #[doc = "Bit 15 - Comparator 1 lock"]
    #[inline(always)]
    #[must_use]
    pub fn comp1lock(&mut self) -> COMP1LOCK_W<15> {
        COMP1LOCK_W::new(self)
    }
    #[doc = "Bit 16 - Comparator 2 enable"]
    #[inline(always)]
    #[must_use]
    pub fn comp2en(&mut self) -> COMP2EN_W<16> {
        COMP2EN_W::new(self)
    }
    #[doc = "Bits 18:19 - Comparator 2 mode"]
    #[inline(always)]
    #[must_use]
    pub fn comp2mode(&mut self) -> COMP2MODE_W<18> {
        COMP2MODE_W::new(self)
    }
    #[doc = "Bits 20:22 - Comparator 2 inverting input selection"]
    #[inline(always)]
    #[must_use]
    pub fn comp2insel(&mut self) -> COMP2INSEL_W<20> {
        COMP2INSEL_W::new(self)
    }
    #[doc = "Bit 23 - Window mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn wndwen(&mut self) -> WNDWEN_W<23> {
        WNDWEN_W::new(self)
    }
    #[doc = "Bits 24:26 - Comparator 2 output selection"]
    #[inline(always)]
    #[must_use]
    pub fn comp2outsel(&mut self) -> COMP2OUTSEL_W<24> {
        COMP2OUTSEL_W::new(self)
    }
    #[doc = "Bit 27 - Comparator 2 output polarity"]
    #[inline(always)]
    #[must_use]
    pub fn comp2pol(&mut self) -> COMP2POL_W<27> {
        COMP2POL_W::new(self)
    }
    #[doc = "Bits 28:29 - Comparator 2 hysteresis"]
    #[inline(always)]
    #[must_use]
    pub fn comp2hyst(&mut self) -> COMP2HYST_W<28> {
        COMP2HYST_W::new(self)
    }
    #[doc = "Bit 31 - Comparator 2 lock"]
    #[inline(always)]
    #[must_use]
    pub fn comp2lock(&mut self) -> COMP2LOCK_W<31> {
        COMP2LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr](index.html) module"]
pub struct CSR_SPEC;
impl crate::RegisterSpec for CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csr::R](R) reader structure"]
impl crate::Readable for CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csr::W](W) writer structure"]
impl crate::Writable for CSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSR to value 0"]
impl crate::Resettable for CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
