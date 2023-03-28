#[doc = "Register `OPFCCR` reader"]
pub struct R(crate::R<OPFCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPFCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPFCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPFCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPFCCR` writer"]
pub struct W(crate::W<OPFCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPFCCR_SPEC>;
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
impl From<crate::W<OPFCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPFCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CM` reader - Color mode These bits define the color format of the output image. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. others: meaningless"]
pub type CM_R = crate::FieldReader<u8, CM_A>;
#[doc = "Color mode These bits define the color format of the output image. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. others: meaningless\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CM_A {
    #[doc = "0: ARGB8888"]
    Argb8888 = 0,
    #[doc = "1: RGB888"]
    Rgb888 = 1,
    #[doc = "2: RGB565"]
    Rgb565 = 2,
    #[doc = "3: ARGB1555"]
    Argb1555 = 3,
    #[doc = "4: ARGB4444"]
    Argb4444 = 4,
}
impl From<CM_A> for u8 {
    #[inline(always)]
    fn from(variant: CM_A) -> Self {
        variant as _
    }
}
impl CM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CM_A> {
        match self.bits {
            0 => Some(CM_A::Argb8888),
            1 => Some(CM_A::Rgb888),
            2 => Some(CM_A::Rgb565),
            3 => Some(CM_A::Argb1555),
            4 => Some(CM_A::Argb4444),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Argb8888`"]
    #[inline(always)]
    pub fn is_argb8888(&self) -> bool {
        *self == CM_A::Argb8888
    }
    #[doc = "Checks if the value of the field is `Rgb888`"]
    #[inline(always)]
    pub fn is_rgb888(&self) -> bool {
        *self == CM_A::Rgb888
    }
    #[doc = "Checks if the value of the field is `Rgb565`"]
    #[inline(always)]
    pub fn is_rgb565(&self) -> bool {
        *self == CM_A::Rgb565
    }
    #[doc = "Checks if the value of the field is `Argb1555`"]
    #[inline(always)]
    pub fn is_argb1555(&self) -> bool {
        *self == CM_A::Argb1555
    }
    #[doc = "Checks if the value of the field is `Argb4444`"]
    #[inline(always)]
    pub fn is_argb4444(&self) -> bool {
        *self == CM_A::Argb4444
    }
}
#[doc = "Field `CM` writer - Color mode These bits define the color format of the output image. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. others: meaningless"]
pub type CM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPFCCR_SPEC, u8, CM_A, 3, O>;
impl<'a, const O: u8> CM_W<'a, O> {
    #[doc = "ARGB8888"]
    #[inline(always)]
    pub fn argb8888(self) -> &'a mut W {
        self.variant(CM_A::Argb8888)
    }
    #[doc = "RGB888"]
    #[inline(always)]
    pub fn rgb888(self) -> &'a mut W {
        self.variant(CM_A::Rgb888)
    }
    #[doc = "RGB565"]
    #[inline(always)]
    pub fn rgb565(self) -> &'a mut W {
        self.variant(CM_A::Rgb565)
    }
    #[doc = "ARGB1555"]
    #[inline(always)]
    pub fn argb1555(self) -> &'a mut W {
        self.variant(CM_A::Argb1555)
    }
    #[doc = "ARGB4444"]
    #[inline(always)]
    pub fn argb4444(self) -> &'a mut W {
        self.variant(CM_A::Argb4444)
    }
}
#[doc = "Field `SB` reader - Swap Bytes"]
pub type SB_R = crate::BitReader<SB_A>;
#[doc = "Swap Bytes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SB_A {
    #[doc = "0: Regular byte order"]
    Regular = 0,
    #[doc = "1: Bytes are swapped two by two"]
    SwapBytes = 1,
}
impl From<SB_A> for bool {
    #[inline(always)]
    fn from(variant: SB_A) -> Self {
        variant as u8 != 0
    }
}
impl SB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SB_A {
        match self.bits {
            false => SB_A::Regular,
            true => SB_A::SwapBytes,
        }
    }
    #[doc = "Checks if the value of the field is `Regular`"]
    #[inline(always)]
    pub fn is_regular(&self) -> bool {
        *self == SB_A::Regular
    }
    #[doc = "Checks if the value of the field is `SwapBytes`"]
    #[inline(always)]
    pub fn is_swap_bytes(&self) -> bool {
        *self == SB_A::SwapBytes
    }
}
#[doc = "Field `SB` writer - Swap Bytes"]
pub type SB_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPFCCR_SPEC, SB_A, O>;
impl<'a, const O: u8> SB_W<'a, O> {
    #[doc = "Regular byte order"]
    #[inline(always)]
    pub fn regular(self) -> &'a mut W {
        self.variant(SB_A::Regular)
    }
    #[doc = "Bytes are swapped two by two"]
    #[inline(always)]
    pub fn swap_bytes(self) -> &'a mut W {
        self.variant(SB_A::SwapBytes)
    }
}
#[doc = "Field `AI` reader - Alpha Inverted This bit inverts the alpha value. Once the transfer has started, this bit is read-only."]
pub type AI_R = crate::BitReader<AI_A>;
#[doc = "Alpha Inverted This bit inverts the alpha value. Once the transfer has started, this bit is read-only.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AI_A {
    #[doc = "0: Regular alpha"]
    RegularAlpha = 0,
    #[doc = "1: Inverted alpha"]
    InvertedAlpha = 1,
}
impl From<AI_A> for bool {
    #[inline(always)]
    fn from(variant: AI_A) -> Self {
        variant as u8 != 0
    }
}
impl AI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AI_A {
        match self.bits {
            false => AI_A::RegularAlpha,
            true => AI_A::InvertedAlpha,
        }
    }
    #[doc = "Checks if the value of the field is `RegularAlpha`"]
    #[inline(always)]
    pub fn is_regular_alpha(&self) -> bool {
        *self == AI_A::RegularAlpha
    }
    #[doc = "Checks if the value of the field is `InvertedAlpha`"]
    #[inline(always)]
    pub fn is_inverted_alpha(&self) -> bool {
        *self == AI_A::InvertedAlpha
    }
}
#[doc = "Field `AI` writer - Alpha Inverted This bit inverts the alpha value. Once the transfer has started, this bit is read-only."]
pub type AI_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPFCCR_SPEC, AI_A, O>;
impl<'a, const O: u8> AI_W<'a, O> {
    #[doc = "Regular alpha"]
    #[inline(always)]
    pub fn regular_alpha(self) -> &'a mut W {
        self.variant(AI_A::RegularAlpha)
    }
    #[doc = "Inverted alpha"]
    #[inline(always)]
    pub fn inverted_alpha(self) -> &'a mut W {
        self.variant(AI_A::InvertedAlpha)
    }
}
#[doc = "Field `RBS` reader - Red Blue Swap This bit allows to swap the R &amp; B to support BGR or ABGR color formats. Once the transfer has started, this bit is read-only."]
pub type RBS_R = crate::BitReader<RBS_A>;
#[doc = "Red Blue Swap This bit allows to swap the R &amp; B to support BGR or ABGR color formats. Once the transfer has started, this bit is read-only.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RBS_A {
    #[doc = "0: No Red Blue Swap (RGB or ARGB)"]
    Regular = 0,
    #[doc = "1: Red Blue Swap (BGR or ABGR)"]
    Swap = 1,
}
impl From<RBS_A> for bool {
    #[inline(always)]
    fn from(variant: RBS_A) -> Self {
        variant as u8 != 0
    }
}
impl RBS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RBS_A {
        match self.bits {
            false => RBS_A::Regular,
            true => RBS_A::Swap,
        }
    }
    #[doc = "Checks if the value of the field is `Regular`"]
    #[inline(always)]
    pub fn is_regular(&self) -> bool {
        *self == RBS_A::Regular
    }
    #[doc = "Checks if the value of the field is `Swap`"]
    #[inline(always)]
    pub fn is_swap(&self) -> bool {
        *self == RBS_A::Swap
    }
}
#[doc = "Field `RBS` writer - Red Blue Swap This bit allows to swap the R &amp; B to support BGR or ABGR color formats. Once the transfer has started, this bit is read-only."]
pub type RBS_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPFCCR_SPEC, RBS_A, O>;
impl<'a, const O: u8> RBS_W<'a, O> {
    #[doc = "No Red Blue Swap (RGB or ARGB)"]
    #[inline(always)]
    pub fn regular(self) -> &'a mut W {
        self.variant(RBS_A::Regular)
    }
    #[doc = "Red Blue Swap (BGR or ABGR)"]
    #[inline(always)]
    pub fn swap(self) -> &'a mut W {
        self.variant(RBS_A::Swap)
    }
}
impl R {
    #[doc = "Bits 0:2 - Color mode These bits define the color format of the output image. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. others: meaningless"]
    #[inline(always)]
    pub fn cm(&self) -> CM_R {
        CM_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - Swap Bytes"]
    #[inline(always)]
    pub fn sb(&self) -> SB_R {
        SB_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 20 - Alpha Inverted This bit inverts the alpha value. Once the transfer has started, this bit is read-only."]
    #[inline(always)]
    pub fn ai(&self) -> AI_R {
        AI_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Red Blue Swap This bit allows to swap the R &amp; B to support BGR or ABGR color formats. Once the transfer has started, this bit is read-only."]
    #[inline(always)]
    pub fn rbs(&self) -> RBS_R {
        RBS_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Color mode These bits define the color format of the output image. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. others: meaningless"]
    #[inline(always)]
    #[must_use]
    pub fn cm(&mut self) -> CM_W<0> {
        CM_W::new(self)
    }
    #[doc = "Bit 8 - Swap Bytes"]
    #[inline(always)]
    #[must_use]
    pub fn sb(&mut self) -> SB_W<8> {
        SB_W::new(self)
    }
    #[doc = "Bit 20 - Alpha Inverted This bit inverts the alpha value. Once the transfer has started, this bit is read-only."]
    #[inline(always)]
    #[must_use]
    pub fn ai(&mut self) -> AI_W<20> {
        AI_W::new(self)
    }
    #[doc = "Bit 21 - Red Blue Swap This bit allows to swap the R &amp; B to support BGR or ABGR color formats. Once the transfer has started, this bit is read-only."]
    #[inline(always)]
    #[must_use]
    pub fn rbs(&mut self) -> RBS_W<21> {
        RBS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA2D output PFC control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opfccr](index.html) module"]
pub struct OPFCCR_SPEC;
impl crate::RegisterSpec for OPFCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [opfccr::R](R) reader structure"]
impl crate::Readable for OPFCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [opfccr::W](W) writer structure"]
impl crate::Writable for OPFCCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OPFCCR to value 0"]
impl crate::Resettable for OPFCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
