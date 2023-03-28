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
#[doc = "Field `EN` reader - COMP channel 1 enable bit"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - COMP channel 1 enable bit"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP1_CSR_SPEC, bool, O>;
#[doc = "Field `INMSEL` reader - Comparator 2 signal selector for inverting input INM"]
pub type INMSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INMSEL` writer - Comparator 2 signal selector for inverting input INM"]
pub type INMSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COMP1_CSR_SPEC, u8, u8, 4, O>;
#[doc = "Field `INPSEL` reader - Comparator 2 signal selector for non-inverting input"]
pub type INPSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INPSEL` writer - Comparator 2 signal selector for non-inverting input"]
pub type INPSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COMP1_CSR_SPEC, u8, u8, 2, O>;
#[doc = "Field `WINMODE` reader - Comparator 2 non-inverting input selector for window mode"]
pub type WINMODE_R = crate::BitReader<bool>;
#[doc = "Field `WINMODE` writer - Comparator 2 non-inverting input selector for window mode"]
pub type WINMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP1_CSR_SPEC, bool, O>;
#[doc = "Field `WINOUT` reader - Comparator 2 output selector"]
pub type WINOUT_R = crate::BitReader<bool>;
#[doc = "Field `WINOUT` writer - Comparator 2 output selector"]
pub type WINOUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP1_CSR_SPEC, bool, O>;
#[doc = "Field `POLARITY` reader - Comparator 2 polarity selector"]
pub type POLARITY_R = crate::BitReader<bool>;
#[doc = "Field `POLARITY` writer - Comparator 2 polarity selector"]
pub type POLARITY_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP1_CSR_SPEC, bool, O>;
#[doc = "Field `HYST` reader - Comparator 2 hysteresis selector"]
pub type HYST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HYST` writer - Comparator 2 hysteresis selector"]
pub type HYST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COMP1_CSR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PWRMODE` reader - Comparator 2 power mode selector"]
pub type PWRMODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PWRMODE` writer - Comparator 2 power mode selector"]
pub type PWRMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COMP1_CSR_SPEC, u8, u8, 2, O>;
#[doc = "Field `BLANKSEL` reader - Comparator 2 blanking source selector"]
pub type BLANKSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BLANKSEL` writer - Comparator 2 blanking source selector"]
pub type BLANKSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COMP1_CSR_SPEC, u8, u8, 5, O>;
#[doc = "Field `VALUE` reader - Comparator 2 output status"]
pub type VALUE_R = crate::BitReader<bool>;
#[doc = "Field `VALUE` writer - Comparator 2 output status"]
pub type VALUE_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP1_CSR_SPEC, bool, O>;
#[doc = "Field `LOCK` reader - COMP2_CSR register lock"]
pub type LOCK_R = crate::BitReader<bool>;
#[doc = "Field `LOCK` writer - COMP2_CSR register lock"]
pub type LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP1_CSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - COMP channel 1 enable bit"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:7 - Comparator 2 signal selector for inverting input INM"]
    #[inline(always)]
    pub fn inmsel(&self) -> INMSEL_R {
        INMSEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Comparator 2 signal selector for non-inverting input"]
    #[inline(always)]
    pub fn inpsel(&self) -> INPSEL_R {
        INPSEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 11 - Comparator 2 non-inverting input selector for window mode"]
    #[inline(always)]
    pub fn winmode(&self) -> WINMODE_R {
        WINMODE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - Comparator 2 output selector"]
    #[inline(always)]
    pub fn winout(&self) -> WINOUT_R {
        WINOUT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Comparator 2 polarity selector"]
    #[inline(always)]
    pub fn polarity(&self) -> POLARITY_R {
        POLARITY_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Comparator 2 hysteresis selector"]
    #[inline(always)]
    pub fn hyst(&self) -> HYST_R {
        HYST_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Comparator 2 power mode selector"]
    #[inline(always)]
    pub fn pwrmode(&self) -> PWRMODE_R {
        PWRMODE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:24 - Comparator 2 blanking source selector"]
    #[inline(always)]
    pub fn blanksel(&self) -> BLANKSEL_R {
        BLANKSEL_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bit 30 - Comparator 2 output status"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - COMP2_CSR register lock"]
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
    #[doc = "Bits 4:7 - Comparator 2 signal selector for inverting input INM"]
    #[inline(always)]
    #[must_use]
    pub fn inmsel(&mut self) -> INMSEL_W<4> {
        INMSEL_W::new(self)
    }
    #[doc = "Bits 8:9 - Comparator 2 signal selector for non-inverting input"]
    #[inline(always)]
    #[must_use]
    pub fn inpsel(&mut self) -> INPSEL_W<8> {
        INPSEL_W::new(self)
    }
    #[doc = "Bit 11 - Comparator 2 non-inverting input selector for window mode"]
    #[inline(always)]
    #[must_use]
    pub fn winmode(&mut self) -> WINMODE_W<11> {
        WINMODE_W::new(self)
    }
    #[doc = "Bit 14 - Comparator 2 output selector"]
    #[inline(always)]
    #[must_use]
    pub fn winout(&mut self) -> WINOUT_W<14> {
        WINOUT_W::new(self)
    }
    #[doc = "Bit 15 - Comparator 2 polarity selector"]
    #[inline(always)]
    #[must_use]
    pub fn polarity(&mut self) -> POLARITY_W<15> {
        POLARITY_W::new(self)
    }
    #[doc = "Bits 16:17 - Comparator 2 hysteresis selector"]
    #[inline(always)]
    #[must_use]
    pub fn hyst(&mut self) -> HYST_W<16> {
        HYST_W::new(self)
    }
    #[doc = "Bits 18:19 - Comparator 2 power mode selector"]
    #[inline(always)]
    #[must_use]
    pub fn pwrmode(&mut self) -> PWRMODE_W<18> {
        PWRMODE_W::new(self)
    }
    #[doc = "Bits 20:24 - Comparator 2 blanking source selector"]
    #[inline(always)]
    #[must_use]
    pub fn blanksel(&mut self) -> BLANKSEL_W<20> {
        BLANKSEL_W::new(self)
    }
    #[doc = "Bit 30 - Comparator 2 output status"]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> VALUE_W<30> {
        VALUE_W::new(self)
    }
    #[doc = "Bit 31 - COMP2_CSR register lock"]
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
