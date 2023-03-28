#[doc = "Register `TIMEOUTR` reader"]
pub struct R(crate::R<TIMEOUTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMEOUTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMEOUTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMEOUTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMEOUTR` writer"]
pub struct W(crate::W<TIMEOUTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMEOUTR_SPEC>;
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
impl From<crate::W<TIMEOUTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMEOUTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMEOUTA` reader - Bus Timeout A This field is used to configure: The SCL low timeout condition tTIMEOUT when TIDLE=0 tTIMEOUT= (TIMEOUTA+1) x 2048 x tI2CCLK The bus idle condition (both SCL and SDA high) when TIDLE=1 tIDLE= (TIMEOUTA+1) x 4 x tI2CCLK Note: These bits can be written only when TIMOUTEN=0."]
pub type TIMEOUTA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TIMEOUTA` writer - Bus Timeout A This field is used to configure: The SCL low timeout condition tTIMEOUT when TIDLE=0 tTIMEOUT= (TIMEOUTA+1) x 2048 x tI2CCLK The bus idle condition (both SCL and SDA high) when TIDLE=1 tIDLE= (TIMEOUTA+1) x 4 x tI2CCLK Note: These bits can be written only when TIMOUTEN=0."]
pub type TIMEOUTA_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TIMEOUTR_SPEC, u16, u16, 12, O>;
#[doc = "Field `TIDLE` reader - Idle clock timeout detection Note: This bit can be written only when TIMOUTEN=0."]
pub type TIDLE_R = crate::BitReader<TIDLE_A>;
#[doc = "Idle clock timeout detection Note: This bit can be written only when TIMOUTEN=0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIDLE_A {
    #[doc = "0: TIMEOUTA is used to detect SCL low timeout"]
    Disabled = 0,
    #[doc = "1: TIMEOUTA is used to detect both SCL and SDA high timeout (bus idle condition)"]
    Enabled = 1,
}
impl From<TIDLE_A> for bool {
    #[inline(always)]
    fn from(variant: TIDLE_A) -> Self {
        variant as u8 != 0
    }
}
impl TIDLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIDLE_A {
        match self.bits {
            false => TIDLE_A::Disabled,
            true => TIDLE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIDLE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIDLE_A::Enabled
    }
}
#[doc = "Field `TIDLE` writer - Idle clock timeout detection Note: This bit can be written only when TIMOUTEN=0."]
pub type TIDLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMEOUTR_SPEC, TIDLE_A, O>;
impl<'a, const O: u8> TIDLE_W<'a, O> {
    #[doc = "TIMEOUTA is used to detect SCL low timeout"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIDLE_A::Disabled)
    }
    #[doc = "TIMEOUTA is used to detect both SCL and SDA high timeout (bus idle condition)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIDLE_A::Enabled)
    }
}
#[doc = "Field `TIMOUTEN` reader - Clock timeout enable"]
pub type TIMOUTEN_R = crate::BitReader<TIMOUTEN_A>;
#[doc = "Clock timeout enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMOUTEN_A {
    #[doc = "0: SCL timeout detection is disabled"]
    Disabled = 0,
    #[doc = "1: SCL timeout detection is enabled"]
    Enabled = 1,
}
impl From<TIMOUTEN_A> for bool {
    #[inline(always)]
    fn from(variant: TIMOUTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TIMOUTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMOUTEN_A {
        match self.bits {
            false => TIMOUTEN_A::Disabled,
            true => TIMOUTEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIMOUTEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIMOUTEN_A::Enabled
    }
}
#[doc = "Field `TIMOUTEN` writer - Clock timeout enable"]
pub type TIMOUTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMEOUTR_SPEC, TIMOUTEN_A, O>;
impl<'a, const O: u8> TIMOUTEN_W<'a, O> {
    #[doc = "SCL timeout detection is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIMOUTEN_A::Disabled)
    }
    #[doc = "SCL timeout detection is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIMOUTEN_A::Enabled)
    }
}
#[doc = "Field `TIMEOUTB` reader - Bus timeout B This field is used to configure the cumulative clock extension timeout: In master mode, the master cumulative clock low extend time (tLOW:MEXT) is detected In slave mode, the slave cumulative clock low extend time (tLOW:SEXT) is detected tLOW:EXT= (TIMEOUTB+1) x 2048 x tI2CCLK Note: These bits can be written only when TEXTEN=0."]
pub type TIMEOUTB_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TIMEOUTB` writer - Bus timeout B This field is used to configure the cumulative clock extension timeout: In master mode, the master cumulative clock low extend time (tLOW:MEXT) is detected In slave mode, the slave cumulative clock low extend time (tLOW:SEXT) is detected tLOW:EXT= (TIMEOUTB+1) x 2048 x tI2CCLK Note: These bits can be written only when TEXTEN=0."]
pub type TIMEOUTB_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TIMEOUTR_SPEC, u16, u16, 12, O>;
#[doc = "Field `TEXTEN` reader - Extended clock timeout enable"]
pub type TEXTEN_R = crate::BitReader<TEXTEN_A>;
#[doc = "Extended clock timeout enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEXTEN_A {
    #[doc = "0: Extended clock timeout detection is disabled"]
    Disabled = 0,
    #[doc = "1: Extended clock timeout detection is enabled"]
    Enabled = 1,
}
impl From<TEXTEN_A> for bool {
    #[inline(always)]
    fn from(variant: TEXTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TEXTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEXTEN_A {
        match self.bits {
            false => TEXTEN_A::Disabled,
            true => TEXTEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TEXTEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TEXTEN_A::Enabled
    }
}
#[doc = "Field `TEXTEN` writer - Extended clock timeout enable"]
pub type TEXTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMEOUTR_SPEC, TEXTEN_A, O>;
impl<'a, const O: u8> TEXTEN_W<'a, O> {
    #[doc = "Extended clock timeout detection is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TEXTEN_A::Disabled)
    }
    #[doc = "Extended clock timeout detection is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TEXTEN_A::Enabled)
    }
}
impl R {
    #[doc = "Bits 0:11 - Bus Timeout A This field is used to configure: The SCL low timeout condition tTIMEOUT when TIDLE=0 tTIMEOUT= (TIMEOUTA+1) x 2048 x tI2CCLK The bus idle condition (both SCL and SDA high) when TIDLE=1 tIDLE= (TIMEOUTA+1) x 4 x tI2CCLK Note: These bits can be written only when TIMOUTEN=0."]
    #[inline(always)]
    pub fn timeouta(&self) -> TIMEOUTA_R {
        TIMEOUTA_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12 - Idle clock timeout detection Note: This bit can be written only when TIMOUTEN=0."]
    #[inline(always)]
    pub fn tidle(&self) -> TIDLE_R {
        TIDLE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - Clock timeout enable"]
    #[inline(always)]
    pub fn timouten(&self) -> TIMOUTEN_R {
        TIMOUTEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:27 - Bus timeout B This field is used to configure the cumulative clock extension timeout: In master mode, the master cumulative clock low extend time (tLOW:MEXT) is detected In slave mode, the slave cumulative clock low extend time (tLOW:SEXT) is detected tLOW:EXT= (TIMEOUTB+1) x 2048 x tI2CCLK Note: These bits can be written only when TEXTEN=0."]
    #[inline(always)]
    pub fn timeoutb(&self) -> TIMEOUTB_R {
        TIMEOUTB_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 31 - Extended clock timeout enable"]
    #[inline(always)]
    pub fn texten(&self) -> TEXTEN_R {
        TEXTEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Bus Timeout A This field is used to configure: The SCL low timeout condition tTIMEOUT when TIDLE=0 tTIMEOUT= (TIMEOUTA+1) x 2048 x tI2CCLK The bus idle condition (both SCL and SDA high) when TIDLE=1 tIDLE= (TIMEOUTA+1) x 4 x tI2CCLK Note: These bits can be written only when TIMOUTEN=0."]
    #[inline(always)]
    #[must_use]
    pub fn timeouta(&mut self) -> TIMEOUTA_W<0> {
        TIMEOUTA_W::new(self)
    }
    #[doc = "Bit 12 - Idle clock timeout detection Note: This bit can be written only when TIMOUTEN=0."]
    #[inline(always)]
    #[must_use]
    pub fn tidle(&mut self) -> TIDLE_W<12> {
        TIDLE_W::new(self)
    }
    #[doc = "Bit 15 - Clock timeout enable"]
    #[inline(always)]
    #[must_use]
    pub fn timouten(&mut self) -> TIMOUTEN_W<15> {
        TIMOUTEN_W::new(self)
    }
    #[doc = "Bits 16:27 - Bus timeout B This field is used to configure the cumulative clock extension timeout: In master mode, the master cumulative clock low extend time (tLOW:MEXT) is detected In slave mode, the slave cumulative clock low extend time (tLOW:SEXT) is detected tLOW:EXT= (TIMEOUTB+1) x 2048 x tI2CCLK Note: These bits can be written only when TEXTEN=0."]
    #[inline(always)]
    #[must_use]
    pub fn timeoutb(&mut self) -> TIMEOUTB_W<16> {
        TIMEOUTB_W::new(self)
    }
    #[doc = "Bit 31 - Extended clock timeout enable"]
    #[inline(always)]
    #[must_use]
    pub fn texten(&mut self) -> TEXTEN_W<31> {
        TEXTEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Access: No wait states, except if a write access occurs while a write access to this register is ongoing. In this case, wait states are inserted in the second write access until the previous one is completed. The latency of the second write access can be up to 2 x PCLK1 + 6 x I2CCLK.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timeoutr](index.html) module"]
pub struct TIMEOUTR_SPEC;
impl crate::RegisterSpec for TIMEOUTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timeoutr::R](R) reader structure"]
impl crate::Readable for TIMEOUTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timeoutr::W](W) writer structure"]
impl crate::Writable for TIMEOUTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMEOUTR to value 0"]
impl crate::Resettable for TIMEOUTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
