#[doc = "Register `OFR4` reader"]
pub struct R(crate::R<OFR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OFR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OFR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OFR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OFR4` writer"]
pub struct W(crate::W<OFR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OFR4_SPEC>;
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
impl From<crate::W<OFR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OFR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OFFSET4` reader - Data offset 4 for the channel programmed into bits OFFSET4_CH"]
pub type OFFSET4_R = crate::FieldReader<u16, u16>;
#[doc = "Field `OFFSET4` writer - Data offset 4 for the channel programmed into bits OFFSET4_CH"]
pub type OFFSET4_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, OFR4_SPEC, u16, u16, 12, O>;
#[doc = "Field `OFFSETPOS` reader - Positive offset"]
pub type OFFSETPOS_R = crate::BitReader<bool>;
#[doc = "Field `OFFSETPOS` writer - Positive offset"]
pub type OFFSETPOS_W<'a, const O: u8> = crate::BitWriter<'a, u32, OFR4_SPEC, bool, O>;
#[doc = "Field `SATEN` reader - Saturation enable"]
pub type SATEN_R = crate::BitReader<bool>;
#[doc = "Field `SATEN` writer - Saturation enable"]
pub type SATEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OFR4_SPEC, bool, O>;
#[doc = "Field `OFFSET4_CH` reader - Channel selection for the data offset 4"]
pub type OFFSET4_CH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OFFSET4_CH` writer - Channel selection for the data offset 4"]
pub type OFFSET4_CH_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, OFR4_SPEC, u8, u8, 5, O>;
#[doc = "Field `OFFSET4_EN` reader - Offset 4 Enable"]
pub type OFFSET4_EN_R = crate::BitReader<OFFSET1_EN_A>;
#[doc = "Offset 4 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OFFSET1_EN_A {
    #[doc = "0: Offset disabled"]
    Disabled = 0,
    #[doc = "1: Offset enabled"]
    Enabled = 1,
}
impl From<OFFSET1_EN_A> for bool {
    #[inline(always)]
    fn from(variant: OFFSET1_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl OFFSET4_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OFFSET1_EN_A {
        match self.bits {
            false => OFFSET1_EN_A::Disabled,
            true => OFFSET1_EN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OFFSET1_EN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OFFSET1_EN_A::Enabled
    }
}
#[doc = "Field `OFFSET4_EN` writer - Offset 4 Enable"]
pub type OFFSET4_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OFR4_SPEC, OFFSET1_EN_A, O>;
impl<'a, const O: u8> OFFSET4_EN_W<'a, O> {
    #[doc = "Offset disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OFFSET1_EN_A::Disabled)
    }
    #[doc = "Offset enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OFFSET1_EN_A::Enabled)
    }
}
impl R {
    #[doc = "Bits 0:11 - Data offset 4 for the channel programmed into bits OFFSET4_CH"]
    #[inline(always)]
    pub fn offset4(&self) -> OFFSET4_R {
        OFFSET4_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 24 - Positive offset"]
    #[inline(always)]
    pub fn offsetpos(&self) -> OFFSETPOS_R {
        OFFSETPOS_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Saturation enable"]
    #[inline(always)]
    pub fn saten(&self) -> SATEN_R {
        SATEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:30 - Channel selection for the data offset 4"]
    #[inline(always)]
    pub fn offset4_ch(&self) -> OFFSET4_CH_R {
        OFFSET4_CH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Offset 4 Enable"]
    #[inline(always)]
    pub fn offset4_en(&self) -> OFFSET4_EN_R {
        OFFSET4_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Data offset 4 for the channel programmed into bits OFFSET4_CH"]
    #[inline(always)]
    #[must_use]
    pub fn offset4(&mut self) -> OFFSET4_W<0> {
        OFFSET4_W::new(self)
    }
    #[doc = "Bit 24 - Positive offset"]
    #[inline(always)]
    #[must_use]
    pub fn offsetpos(&mut self) -> OFFSETPOS_W<24> {
        OFFSETPOS_W::new(self)
    }
    #[doc = "Bit 25 - Saturation enable"]
    #[inline(always)]
    #[must_use]
    pub fn saten(&mut self) -> SATEN_W<25> {
        SATEN_W::new(self)
    }
    #[doc = "Bits 26:30 - Channel selection for the data offset 4"]
    #[inline(always)]
    #[must_use]
    pub fn offset4_ch(&mut self) -> OFFSET4_CH_W<26> {
        OFFSET4_CH_W::new(self)
    }
    #[doc = "Bit 31 - Offset 4 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn offset4_en(&mut self) -> OFFSET4_EN_W<31> {
        OFFSET4_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "offset register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ofr4](index.html) module"]
pub struct OFR4_SPEC;
impl crate::RegisterSpec for OFR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ofr4::R](R) reader structure"]
impl crate::Readable for OFR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ofr4::W](W) writer structure"]
impl crate::Writable for OFR4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OFR4 to value 0"]
impl crate::Resettable for OFR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
