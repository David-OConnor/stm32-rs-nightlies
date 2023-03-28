#[doc = "Register `IER` reader"]
pub struct R(crate::R<IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IER` writer"]
pub struct W(crate::W<IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IER_SPEC>;
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
impl From<crate::W<IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRAME_IE` reader - Capture complete interrupt enable"]
pub type FRAME_IE_R = crate::BitReader<FRAME_IE_A>;
#[doc = "Capture complete interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRAME_IE_A {
    #[doc = "0: No interrupt generation"]
    Disabled = 0,
    #[doc = "1: An interrupt is generated at the end of each received frame/crop window (in crop mode)"]
    Enabled = 1,
}
impl From<FRAME_IE_A> for bool {
    #[inline(always)]
    fn from(variant: FRAME_IE_A) -> Self {
        variant as u8 != 0
    }
}
impl FRAME_IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRAME_IE_A {
        match self.bits {
            false => FRAME_IE_A::Disabled,
            true => FRAME_IE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FRAME_IE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FRAME_IE_A::Enabled
    }
}
#[doc = "Field `FRAME_IE` writer - Capture complete interrupt enable"]
pub type FRAME_IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, FRAME_IE_A, O>;
impl<'a, const O: u8> FRAME_IE_W<'a, O> {
    #[doc = "No interrupt generation"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FRAME_IE_A::Disabled)
    }
    #[doc = "An interrupt is generated at the end of each received frame/crop window (in crop mode)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FRAME_IE_A::Enabled)
    }
}
#[doc = "Field `OVR_IE` reader - Overrun interrupt enable"]
pub type OVR_IE_R = crate::BitReader<OVR_IE_A>;
#[doc = "Overrun interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVR_IE_A {
    #[doc = "0: No interrupt generation"]
    Disabled = 0,
    #[doc = "1: An interrupt is generated if the DMA was not able to transfer the last data before new data (32-bit) are received"]
    Enabled = 1,
}
impl From<OVR_IE_A> for bool {
    #[inline(always)]
    fn from(variant: OVR_IE_A) -> Self {
        variant as u8 != 0
    }
}
impl OVR_IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVR_IE_A {
        match self.bits {
            false => OVR_IE_A::Disabled,
            true => OVR_IE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OVR_IE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OVR_IE_A::Enabled
    }
}
#[doc = "Field `OVR_IE` writer - Overrun interrupt enable"]
pub type OVR_IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, OVR_IE_A, O>;
impl<'a, const O: u8> OVR_IE_W<'a, O> {
    #[doc = "No interrupt generation"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OVR_IE_A::Disabled)
    }
    #[doc = "An interrupt is generated if the DMA was not able to transfer the last data before new data (32-bit) are received"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OVR_IE_A::Enabled)
    }
}
#[doc = "Field `ERR_IE` reader - Synchronization error interrupt enable"]
pub type ERR_IE_R = crate::BitReader<ERR_IE_A>;
#[doc = "Synchronization error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERR_IE_A {
    #[doc = "0: No interrupt generation"]
    Disabled = 0,
    #[doc = "1: An interrupt is generated if the embedded synchronization codes are not received in the correct order"]
    Enabled = 1,
}
impl From<ERR_IE_A> for bool {
    #[inline(always)]
    fn from(variant: ERR_IE_A) -> Self {
        variant as u8 != 0
    }
}
impl ERR_IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR_IE_A {
        match self.bits {
            false => ERR_IE_A::Disabled,
            true => ERR_IE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ERR_IE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ERR_IE_A::Enabled
    }
}
#[doc = "Field `ERR_IE` writer - Synchronization error interrupt enable"]
pub type ERR_IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, ERR_IE_A, O>;
impl<'a, const O: u8> ERR_IE_W<'a, O> {
    #[doc = "No interrupt generation"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ERR_IE_A::Disabled)
    }
    #[doc = "An interrupt is generated if the embedded synchronization codes are not received in the correct order"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ERR_IE_A::Enabled)
    }
}
#[doc = "Field `VSYNC_IE` reader - VSYNC interrupt enable"]
pub type VSYNC_IE_R = crate::BitReader<VSYNC_IE_A>;
#[doc = "VSYNC interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VSYNC_IE_A {
    #[doc = "0: No interrupt generation"]
    Disabled = 0,
    #[doc = "1: An interrupt is generated on each DCMI_VSYNC transition from the inactive to the active state"]
    Enabled = 1,
}
impl From<VSYNC_IE_A> for bool {
    #[inline(always)]
    fn from(variant: VSYNC_IE_A) -> Self {
        variant as u8 != 0
    }
}
impl VSYNC_IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VSYNC_IE_A {
        match self.bits {
            false => VSYNC_IE_A::Disabled,
            true => VSYNC_IE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VSYNC_IE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VSYNC_IE_A::Enabled
    }
}
#[doc = "Field `VSYNC_IE` writer - VSYNC interrupt enable"]
pub type VSYNC_IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, VSYNC_IE_A, O>;
impl<'a, const O: u8> VSYNC_IE_W<'a, O> {
    #[doc = "No interrupt generation"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(VSYNC_IE_A::Disabled)
    }
    #[doc = "An interrupt is generated on each DCMI_VSYNC transition from the inactive to the active state"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(VSYNC_IE_A::Enabled)
    }
}
#[doc = "Field `LINE_IE` reader - Line interrupt enable"]
pub type LINE_IE_R = crate::BitReader<LINE_IE_A>;
#[doc = "Line interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LINE_IE_A {
    #[doc = "0: No interrupt generation when the line is received"]
    Disabled = 0,
    #[doc = "1: An Interrupt is generated when a line has been completely received"]
    Enabled = 1,
}
impl From<LINE_IE_A> for bool {
    #[inline(always)]
    fn from(variant: LINE_IE_A) -> Self {
        variant as u8 != 0
    }
}
impl LINE_IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINE_IE_A {
        match self.bits {
            false => LINE_IE_A::Disabled,
            true => LINE_IE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LINE_IE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LINE_IE_A::Enabled
    }
}
#[doc = "Field `LINE_IE` writer - Line interrupt enable"]
pub type LINE_IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, LINE_IE_A, O>;
impl<'a, const O: u8> LINE_IE_W<'a, O> {
    #[doc = "No interrupt generation when the line is received"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LINE_IE_A::Disabled)
    }
    #[doc = "An Interrupt is generated when a line has been completely received"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LINE_IE_A::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Capture complete interrupt enable"]
    #[inline(always)]
    pub fn frame_ie(&self) -> FRAME_IE_R {
        FRAME_IE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Overrun interrupt enable"]
    #[inline(always)]
    pub fn ovr_ie(&self) -> OVR_IE_R {
        OVR_IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Synchronization error interrupt enable"]
    #[inline(always)]
    pub fn err_ie(&self) -> ERR_IE_R {
        ERR_IE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VSYNC interrupt enable"]
    #[inline(always)]
    pub fn vsync_ie(&self) -> VSYNC_IE_R {
        VSYNC_IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Line interrupt enable"]
    #[inline(always)]
    pub fn line_ie(&self) -> LINE_IE_R {
        LINE_IE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Capture complete interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn frame_ie(&mut self) -> FRAME_IE_W<0> {
        FRAME_IE_W::new(self)
    }
    #[doc = "Bit 1 - Overrun interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovr_ie(&mut self) -> OVR_IE_W<1> {
        OVR_IE_W::new(self)
    }
    #[doc = "Bit 2 - Synchronization error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn err_ie(&mut self) -> ERR_IE_W<2> {
        ERR_IE_W::new(self)
    }
    #[doc = "Bit 3 - VSYNC interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn vsync_ie(&mut self) -> VSYNC_IE_W<3> {
        VSYNC_IE_W::new(self)
    }
    #[doc = "Bit 4 - Line interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn line_ie(&mut self) -> LINE_IE_W<4> {
        LINE_IE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](index.html) module"]
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ier::R](R) reader structure"]
impl crate::Readable for IER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ier::W](W) writer structure"]
impl crate::Writable for IER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
