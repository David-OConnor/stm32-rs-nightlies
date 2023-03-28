#[doc = "Register `CR2` reader"]
pub struct R(crate::R<CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR2` writer"]
pub struct W(crate::W<CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR2_SPEC>;
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
impl From<crate::W<CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PVDE` reader - Power voltage detector enable"]
pub type PVDE_R = crate::BitReader<bool>;
#[doc = "Field `PVDE` writer - Power voltage detector enable"]
pub type PVDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `PLS` reader - Power voltage detector level selection"]
pub type PLS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLS` writer - Power voltage detector level selection"]
pub type PLS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR2_SPEC, u8, u8, 3, O>;
#[doc = "Field `PVMEN1` reader - Peripheral voltage monitoring 1 enable: VDDA vs. COMP min voltage"]
pub type PVMEN1_R = crate::BitReader<bool>;
#[doc = "Field `PVMEN1` writer - Peripheral voltage monitoring 1 enable: VDDA vs. COMP min voltage"]
pub type PVMEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `PVMEN2` reader - Peripheral voltage monitoring 2 enable: VDDA vs. Fast DAC min voltage"]
pub type PVMEN2_R = crate::BitReader<bool>;
#[doc = "Field `PVMEN2` writer - Peripheral voltage monitoring 2 enable: VDDA vs. Fast DAC min voltage"]
pub type PVMEN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `PVMEN3` reader - Peripheral voltage monitoring 3 enable: VDDA vs. ADC min voltage 1.62V"]
pub type PVMEN3_R = crate::BitReader<bool>;
#[doc = "Field `PVMEN3` writer - Peripheral voltage monitoring 3 enable: VDDA vs. ADC min voltage 1.62V"]
pub type PVMEN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `PVMEN4` reader - Peripheral voltage monitoring 4 enable: VDDA vs. OPAMP/DAC min voltage"]
pub type PVMEN4_R = crate::BitReader<bool>;
#[doc = "Field `PVMEN4` writer - Peripheral voltage monitoring 4 enable: VDDA vs. OPAMP/DAC min voltage"]
pub type PVMEN4_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Power voltage detector enable"]
    #[inline(always)]
    pub fn pvde(&self) -> PVDE_R {
        PVDE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Power voltage detector level selection"]
    #[inline(always)]
    pub fn pls(&self) -> PLS_R {
        PLS_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - Peripheral voltage monitoring 1 enable: VDDA vs. COMP min voltage"]
    #[inline(always)]
    pub fn pvmen1(&self) -> PVMEN1_R {
        PVMEN1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Peripheral voltage monitoring 2 enable: VDDA vs. Fast DAC min voltage"]
    #[inline(always)]
    pub fn pvmen2(&self) -> PVMEN2_R {
        PVMEN2_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Peripheral voltage monitoring 3 enable: VDDA vs. ADC min voltage 1.62V"]
    #[inline(always)]
    pub fn pvmen3(&self) -> PVMEN3_R {
        PVMEN3_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Peripheral voltage monitoring 4 enable: VDDA vs. OPAMP/DAC min voltage"]
    #[inline(always)]
    pub fn pvmen4(&self) -> PVMEN4_R {
        PVMEN4_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power voltage detector enable"]
    #[inline(always)]
    #[must_use]
    pub fn pvde(&mut self) -> PVDE_W<0> {
        PVDE_W::new(self)
    }
    #[doc = "Bits 1:3 - Power voltage detector level selection"]
    #[inline(always)]
    #[must_use]
    pub fn pls(&mut self) -> PLS_W<1> {
        PLS_W::new(self)
    }
    #[doc = "Bit 4 - Peripheral voltage monitoring 1 enable: VDDA vs. COMP min voltage"]
    #[inline(always)]
    #[must_use]
    pub fn pvmen1(&mut self) -> PVMEN1_W<4> {
        PVMEN1_W::new(self)
    }
    #[doc = "Bit 5 - Peripheral voltage monitoring 2 enable: VDDA vs. Fast DAC min voltage"]
    #[inline(always)]
    #[must_use]
    pub fn pvmen2(&mut self) -> PVMEN2_W<5> {
        PVMEN2_W::new(self)
    }
    #[doc = "Bit 6 - Peripheral voltage monitoring 3 enable: VDDA vs. ADC min voltage 1.62V"]
    #[inline(always)]
    #[must_use]
    pub fn pvmen3(&mut self) -> PVMEN3_W<6> {
        PVMEN3_W::new(self)
    }
    #[doc = "Bit 7 - Peripheral voltage monitoring 4 enable: VDDA vs. OPAMP/DAC min voltage"]
    #[inline(always)]
    #[must_use]
    pub fn pvmen4(&mut self) -> PVMEN4_W<7> {
        PVMEN4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr2](index.html) module"]
pub struct CR2_SPEC;
impl crate::RegisterSpec for CR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr2::R](R) reader structure"]
impl crate::Readable for CR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr2::W](W) writer structure"]
impl crate::Writable for CR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
