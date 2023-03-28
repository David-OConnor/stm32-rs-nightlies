#[doc = "Register `CCR` reader"]
pub struct R(crate::R<CCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCR` writer"]
pub struct W(crate::W<CCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCR_SPEC>;
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
impl From<crate::W<CCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRESC` reader - ADC prescaler Set and cleared by software to select the frequency of the clock to the ADC. Other: Reserved Note: Software is allowed to write these bits only when the ADC is disabled (ADCALÂ =Â 0, ADSTARTÂ =Â 0, ADSTPÂ =Â 0, ADDISÂ =Â 0 and ADENÂ =Â 0)."]
pub type PRESC_R = crate::FieldReader<u8, PRESC_A>;
#[doc = "ADC prescaler Set and cleared by software to select the frequency of the clock to the ADC. Other: Reserved Note: Software is allowed to write these bits only when the ADC is disabled (ADCALÂ =Â 0, ADSTARTÂ =Â 0, ADSTPÂ =Â 0, ADDISÂ =Â 0 and ADENÂ =Â 0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRESC_A {
    #[doc = "0: Input ADC clock not divided"]
    Div1 = 0,
    #[doc = "1: Input ADC clock divided by 2"]
    Div2 = 1,
    #[doc = "2: Input ADC clock divided by 4"]
    Div4 = 2,
    #[doc = "3: Input ADC clock divided by 6"]
    Div6 = 3,
    #[doc = "4: Input ADC clock divided by 8"]
    Div8 = 4,
    #[doc = "5: Input ADC clock divided by 10"]
    Div10 = 5,
    #[doc = "6: Input ADC clock divided by 12"]
    Div12 = 6,
    #[doc = "7: Input ADC clock divided by 16"]
    Div16 = 7,
    #[doc = "8: Input ADC clock divided by 32"]
    Div32 = 8,
    #[doc = "9: Input ADC clock divided by 64"]
    Div64 = 9,
    #[doc = "10: Input ADC clock divided by 128"]
    Div128 = 10,
    #[doc = "11: Input ADC clock divided by 256"]
    Div256 = 11,
}
impl From<PRESC_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESC_A) -> Self {
        variant as _
    }
}
impl PRESC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PRESC_A> {
        match self.bits {
            0 => Some(PRESC_A::Div1),
            1 => Some(PRESC_A::Div2),
            2 => Some(PRESC_A::Div4),
            3 => Some(PRESC_A::Div6),
            4 => Some(PRESC_A::Div8),
            5 => Some(PRESC_A::Div10),
            6 => Some(PRESC_A::Div12),
            7 => Some(PRESC_A::Div16),
            8 => Some(PRESC_A::Div32),
            9 => Some(PRESC_A::Div64),
            10 => Some(PRESC_A::Div128),
            11 => Some(PRESC_A::Div256),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Div1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PRESC_A::Div1
    }
    #[doc = "Checks if the value of the field is `Div2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PRESC_A::Div2
    }
    #[doc = "Checks if the value of the field is `Div4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PRESC_A::Div4
    }
    #[doc = "Checks if the value of the field is `Div6`"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PRESC_A::Div6
    }
    #[doc = "Checks if the value of the field is `Div8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PRESC_A::Div8
    }
    #[doc = "Checks if the value of the field is `Div10`"]
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == PRESC_A::Div10
    }
    #[doc = "Checks if the value of the field is `Div12`"]
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == PRESC_A::Div12
    }
    #[doc = "Checks if the value of the field is `Div16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PRESC_A::Div16
    }
    #[doc = "Checks if the value of the field is `Div32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PRESC_A::Div32
    }
    #[doc = "Checks if the value of the field is `Div64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == PRESC_A::Div64
    }
    #[doc = "Checks if the value of the field is `Div128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == PRESC_A::Div128
    }
    #[doc = "Checks if the value of the field is `Div256`"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == PRESC_A::Div256
    }
}
#[doc = "Field `PRESC` writer - ADC prescaler Set and cleared by software to select the frequency of the clock to the ADC. Other: Reserved Note: Software is allowed to write these bits only when the ADC is disabled (ADCALÂ =Â 0, ADSTARTÂ =Â 0, ADSTPÂ =Â 0, ADDISÂ =Â 0 and ADENÂ =Â 0)."]
pub type PRESC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCR_SPEC, u8, PRESC_A, 4, O>;
impl<'a, const O: u8> PRESC_W<'a, O> {
    #[doc = "Input ADC clock not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PRESC_A::Div1)
    }
    #[doc = "Input ADC clock divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PRESC_A::Div2)
    }
    #[doc = "Input ADC clock divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PRESC_A::Div4)
    }
    #[doc = "Input ADC clock divided by 6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(PRESC_A::Div6)
    }
    #[doc = "Input ADC clock divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PRESC_A::Div8)
    }
    #[doc = "Input ADC clock divided by 10"]
    #[inline(always)]
    pub fn div10(self) -> &'a mut W {
        self.variant(PRESC_A::Div10)
    }
    #[doc = "Input ADC clock divided by 12"]
    #[inline(always)]
    pub fn div12(self) -> &'a mut W {
        self.variant(PRESC_A::Div12)
    }
    #[doc = "Input ADC clock divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PRESC_A::Div16)
    }
    #[doc = "Input ADC clock divided by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(PRESC_A::Div32)
    }
    #[doc = "Input ADC clock divided by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(PRESC_A::Div64)
    }
    #[doc = "Input ADC clock divided by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(PRESC_A::Div128)
    }
    #[doc = "Input ADC clock divided by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(PRESC_A::Div256)
    }
}
#[doc = "Field `VREFEN` reader - VREFINT enable This bit is set and cleared by software to enable/disable the VREFINT. Note: Software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
pub type VREFEN_R = crate::BitReader<VREFEN_A>;
#[doc = "VREFINT enable This bit is set and cleared by software to enable/disable the VREFINT. Note: Software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VREFEN_A {
    #[doc = "0: VREFINT disabled"]
    Disabled = 0,
    #[doc = "1: VREFINT enabled"]
    Enabled = 1,
}
impl From<VREFEN_A> for bool {
    #[inline(always)]
    fn from(variant: VREFEN_A) -> Self {
        variant as u8 != 0
    }
}
impl VREFEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VREFEN_A {
        match self.bits {
            false => VREFEN_A::Disabled,
            true => VREFEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VREFEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VREFEN_A::Enabled
    }
}
#[doc = "Field `VREFEN` writer - VREFINT enable This bit is set and cleared by software to enable/disable the VREFINT. Note: Software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
pub type VREFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, VREFEN_A, O>;
impl<'a, const O: u8> VREFEN_W<'a, O> {
    #[doc = "VREFINT disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(VREFEN_A::Disabled)
    }
    #[doc = "VREFINT enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(VREFEN_A::Enabled)
    }
}
#[doc = "Field `TSEN` reader - Temperature sensor enable This bit is set and cleared by software to enable/disable the temperature sensor. Note: Software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
pub type TSEN_R = crate::BitReader<TSEN_A>;
#[doc = "Temperature sensor enable This bit is set and cleared by software to enable/disable the temperature sensor. Note: Software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSEN_A {
    #[doc = "0: Temperature sensor disabled"]
    Disabled = 0,
    #[doc = "1: Temperature sensor enabled"]
    Enabled = 1,
}
impl From<TSEN_A> for bool {
    #[inline(always)]
    fn from(variant: TSEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSEN_A {
        match self.bits {
            false => TSEN_A::Disabled,
            true => TSEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TSEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TSEN_A::Enabled
    }
}
#[doc = "Field `TSEN` writer - Temperature sensor enable This bit is set and cleared by software to enable/disable the temperature sensor. Note: Software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
pub type TSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, TSEN_A, O>;
impl<'a, const O: u8> TSEN_W<'a, O> {
    #[doc = "Temperature sensor disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TSEN_A::Disabled)
    }
    #[doc = "Temperature sensor enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TSEN_A::Enabled)
    }
}
#[doc = "Field `VBATEN` reader - VBAT enable This bit is set and cleared by software to enable/disable the VBAT channel. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)"]
pub type VBATEN_R = crate::BitReader<VBATEN_A>;
#[doc = "VBAT enable This bit is set and cleared by software to enable/disable the VBAT channel. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBATEN_A {
    #[doc = "0: VBAT channel disabled"]
    Disabled = 0,
    #[doc = "1: VBAT channel enabled"]
    Enabled = 1,
}
impl From<VBATEN_A> for bool {
    #[inline(always)]
    fn from(variant: VBATEN_A) -> Self {
        variant as u8 != 0
    }
}
impl VBATEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBATEN_A {
        match self.bits {
            false => VBATEN_A::Disabled,
            true => VBATEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VBATEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VBATEN_A::Enabled
    }
}
#[doc = "Field `VBATEN` writer - VBAT enable This bit is set and cleared by software to enable/disable the VBAT channel. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)"]
pub type VBATEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, VBATEN_A, O>;
impl<'a, const O: u8> VBATEN_W<'a, O> {
    #[doc = "VBAT channel disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(VBATEN_A::Disabled)
    }
    #[doc = "VBAT channel enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(VBATEN_A::Enabled)
    }
}
impl R {
    #[doc = "Bits 18:21 - ADC prescaler Set and cleared by software to select the frequency of the clock to the ADC. Other: Reserved Note: Software is allowed to write these bits only when the ADC is disabled (ADCALÂ =Â 0, ADSTARTÂ =Â 0, ADSTPÂ =Â 0, ADDISÂ =Â 0 and ADENÂ =Â 0)."]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bit 22 - VREFINT enable This bit is set and cleared by software to enable/disable the VREFINT. Note: Software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn vrefen(&self) -> VREFEN_R {
        VREFEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Temperature sensor enable This bit is set and cleared by software to enable/disable the temperature sensor. Note: Software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    pub fn tsen(&self) -> TSEN_R {
        TSEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - VBAT enable This bit is set and cleared by software to enable/disable the VBAT channel. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)"]
    #[inline(always)]
    pub fn vbaten(&self) -> VBATEN_R {
        VBATEN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 18:21 - ADC prescaler Set and cleared by software to select the frequency of the clock to the ADC. Other: Reserved Note: Software is allowed to write these bits only when the ADC is disabled (ADCALÂ =Â 0, ADSTARTÂ =Â 0, ADSTPÂ =Â 0, ADDISÂ =Â 0 and ADENÂ =Â 0)."]
    #[inline(always)]
    #[must_use]
    pub fn presc(&mut self) -> PRESC_W<18> {
        PRESC_W::new(self)
    }
    #[doc = "Bit 22 - VREFINT enable This bit is set and cleared by software to enable/disable the VREFINT. Note: Software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn vrefen(&mut self) -> VREFEN_W<22> {
        VREFEN_W::new(self)
    }
    #[doc = "Bit 23 - Temperature sensor enable This bit is set and cleared by software to enable/disable the temperature sensor. Note: Software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)."]
    #[inline(always)]
    #[must_use]
    pub fn tsen(&mut self) -> TSEN_W<23> {
        TSEN_W::new(self)
    }
    #[doc = "Bit 24 - VBAT enable This bit is set and cleared by software to enable/disable the VBAT channel. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing)"]
    #[inline(always)]
    #[must_use]
    pub fn vbaten(&mut self) -> VBATEN_W<24> {
        VBATEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC common configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr](index.html) module"]
pub struct CCR_SPEC;
impl crate::RegisterSpec for CCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccr::R](R) reader structure"]
impl crate::Readable for CCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccr::W](W) writer structure"]
impl crate::Writable for CCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCR to value 0"]
impl crate::Resettable for CCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
