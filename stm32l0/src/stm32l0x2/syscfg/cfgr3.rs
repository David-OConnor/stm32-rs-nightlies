#[doc = "Register `CFGR3` reader"]
pub struct R(crate::R<CFGR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR3` writer"]
pub struct W(crate::W<CFGR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR3_SPEC>;
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
impl From<crate::W<CFGR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN_VREFINT` reader - VREFINT enable and scaler control for COMP2 enable bit"]
pub type EN_VREFINT_R = crate::BitReader<EN_VREFINT_A>;
#[doc = "VREFINT enable and scaler control for COMP2 enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN_VREFINT_A {
    #[doc = "0: VREFINT voltage disabled in low-power mode (if ULP=1) and scaler for COMP2 disabled"]
    Disabled = 0,
    #[doc = "1: VREFINT voltage enabled in low-power mode and scaler for COMP2 enabled"]
    Enabled = 1,
}
impl From<EN_VREFINT_A> for bool {
    #[inline(always)]
    fn from(variant: EN_VREFINT_A) -> Self {
        variant as u8 != 0
    }
}
impl EN_VREFINT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_VREFINT_A {
        match self.bits {
            false => EN_VREFINT_A::Disabled,
            true => EN_VREFINT_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EN_VREFINT_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EN_VREFINT_A::Enabled
    }
}
#[doc = "Field `EN_VREFINT` writer - VREFINT enable and scaler control for COMP2 enable bit"]
pub type EN_VREFINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR3_SPEC, EN_VREFINT_A, O>;
impl<'a, const O: u8> EN_VREFINT_W<'a, O> {
    #[doc = "VREFINT voltage disabled in low-power mode (if ULP=1) and scaler for COMP2 disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EN_VREFINT_A::Disabled)
    }
    #[doc = "VREFINT voltage enabled in low-power mode and scaler for COMP2 enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EN_VREFINT_A::Enabled)
    }
}
#[doc = "Field `SEL_VREF_OUT` reader - BGAP_ADC connection bit"]
pub type SEL_VREF_OUT_R = crate::FieldReader<u8, SEL_VREF_OUT_A>;
#[doc = "BGAP_ADC connection bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEL_VREF_OUT_A {
    #[doc = "0: no pad connected"]
    NoConnection = 0,
    #[doc = "1: PB0 connected"]
    Pb0 = 1,
    #[doc = "2: PB1 connected"]
    Pb1 = 2,
    #[doc = "3: PB0 and PB1 connected"]
    Both = 3,
}
impl From<SEL_VREF_OUT_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL_VREF_OUT_A) -> Self {
        variant as _
    }
}
impl SEL_VREF_OUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEL_VREF_OUT_A {
        match self.bits {
            0 => SEL_VREF_OUT_A::NoConnection,
            1 => SEL_VREF_OUT_A::Pb0,
            2 => SEL_VREF_OUT_A::Pb1,
            3 => SEL_VREF_OUT_A::Both,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NoConnection`"]
    #[inline(always)]
    pub fn is_no_connection(&self) -> bool {
        *self == SEL_VREF_OUT_A::NoConnection
    }
    #[doc = "Checks if the value of the field is `Pb0`"]
    #[inline(always)]
    pub fn is_pb0(&self) -> bool {
        *self == SEL_VREF_OUT_A::Pb0
    }
    #[doc = "Checks if the value of the field is `Pb1`"]
    #[inline(always)]
    pub fn is_pb1(&self) -> bool {
        *self == SEL_VREF_OUT_A::Pb1
    }
    #[doc = "Checks if the value of the field is `Both`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == SEL_VREF_OUT_A::Both
    }
}
#[doc = "Field `SEL_VREF_OUT` writer - BGAP_ADC connection bit"]
pub type SEL_VREF_OUT_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CFGR3_SPEC, u8, SEL_VREF_OUT_A, 2, O>;
impl<'a, const O: u8> SEL_VREF_OUT_W<'a, O> {
    #[doc = "no pad connected"]
    #[inline(always)]
    pub fn no_connection(self) -> &'a mut W {
        self.variant(SEL_VREF_OUT_A::NoConnection)
    }
    #[doc = "PB0 connected"]
    #[inline(always)]
    pub fn pb0(self) -> &'a mut W {
        self.variant(SEL_VREF_OUT_A::Pb0)
    }
    #[doc = "PB1 connected"]
    #[inline(always)]
    pub fn pb1(self) -> &'a mut W {
        self.variant(SEL_VREF_OUT_A::Pb1)
    }
    #[doc = "PB0 and PB1 connected"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(SEL_VREF_OUT_A::Both)
    }
}
#[doc = "Field `ENBUF_VREFINT_ADC` reader - VREFINT reference for ADC enable bit"]
pub type ENBUF_VREFINT_ADC_R = crate::BitReader<ENBUF_VREFINT_ADC_A>;
#[doc = "VREFINT reference for ADC enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENBUF_VREFINT_ADC_A {
    #[doc = "0: Disables the buffer used to generate VREFINT reference for the ADC"]
    Disabled = 0,
    #[doc = "1: Enables the buffer used to generate VREFINT reference for the ADC"]
    Enabled = 1,
}
impl From<ENBUF_VREFINT_ADC_A> for bool {
    #[inline(always)]
    fn from(variant: ENBUF_VREFINT_ADC_A) -> Self {
        variant as u8 != 0
    }
}
impl ENBUF_VREFINT_ADC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENBUF_VREFINT_ADC_A {
        match self.bits {
            false => ENBUF_VREFINT_ADC_A::Disabled,
            true => ENBUF_VREFINT_ADC_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENBUF_VREFINT_ADC_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENBUF_VREFINT_ADC_A::Enabled
    }
}
#[doc = "Field `ENBUF_VREFINT_ADC` writer - VREFINT reference for ADC enable bit"]
pub type ENBUF_VREFINT_ADC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CFGR3_SPEC, ENBUF_VREFINT_ADC_A, O>;
impl<'a, const O: u8> ENBUF_VREFINT_ADC_W<'a, O> {
    #[doc = "Disables the buffer used to generate VREFINT reference for the ADC"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENBUF_VREFINT_ADC_A::Disabled)
    }
    #[doc = "Enables the buffer used to generate VREFINT reference for the ADC"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENBUF_VREFINT_ADC_A::Enabled)
    }
}
#[doc = "Field `ENBUF_SENSOR_ADC` reader - Sensor reference for ADC enable bit"]
pub type ENBUF_SENSOR_ADC_R = crate::BitReader<ENBUF_SENSOR_ADC_A>;
#[doc = "Sensor reference for ADC enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENBUF_SENSOR_ADC_A {
    #[doc = "0: Disables the buffer used to generate VREFINT reference for the temperature sensor"]
    Disabled = 0,
    #[doc = "1: Enables the buffer used to generate VREFINT reference for the temperature sensor"]
    Enabled = 1,
}
impl From<ENBUF_SENSOR_ADC_A> for bool {
    #[inline(always)]
    fn from(variant: ENBUF_SENSOR_ADC_A) -> Self {
        variant as u8 != 0
    }
}
impl ENBUF_SENSOR_ADC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENBUF_SENSOR_ADC_A {
        match self.bits {
            false => ENBUF_SENSOR_ADC_A::Disabled,
            true => ENBUF_SENSOR_ADC_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENBUF_SENSOR_ADC_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENBUF_SENSOR_ADC_A::Enabled
    }
}
#[doc = "Field `ENBUF_SENSOR_ADC` writer - Sensor reference for ADC enable bit"]
pub type ENBUF_SENSOR_ADC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CFGR3_SPEC, ENBUF_SENSOR_ADC_A, O>;
impl<'a, const O: u8> ENBUF_SENSOR_ADC_W<'a, O> {
    #[doc = "Disables the buffer used to generate VREFINT reference for the temperature sensor"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENBUF_SENSOR_ADC_A::Disabled)
    }
    #[doc = "Enables the buffer used to generate VREFINT reference for the temperature sensor"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENBUF_SENSOR_ADC_A::Enabled)
    }
}
#[doc = "Field `ENBUF_VREFINT_COMP2` reader - VREFINT reference for COMP2 scaler enable bit"]
pub type ENBUF_VREFINT_COMP2_R = crate::BitReader<ENBUF_VREFINT_COMP2_A>;
#[doc = "VREFINT reference for COMP2 scaler enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENBUF_VREFINT_COMP2_A {
    #[doc = "0: Disables the buffer used to generate VREFINT references for COMP2"]
    Disabled = 0,
    #[doc = "1: Enables the buffer used to generate VREFINT references for COMP2"]
    Enabled = 1,
}
impl From<ENBUF_VREFINT_COMP2_A> for bool {
    #[inline(always)]
    fn from(variant: ENBUF_VREFINT_COMP2_A) -> Self {
        variant as u8 != 0
    }
}
impl ENBUF_VREFINT_COMP2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENBUF_VREFINT_COMP2_A {
        match self.bits {
            false => ENBUF_VREFINT_COMP2_A::Disabled,
            true => ENBUF_VREFINT_COMP2_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENBUF_VREFINT_COMP2_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENBUF_VREFINT_COMP2_A::Enabled
    }
}
#[doc = "Field `ENBUF_VREFINT_COMP2` writer - VREFINT reference for COMP2 scaler enable bit"]
pub type ENBUF_VREFINT_COMP2_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CFGR3_SPEC, ENBUF_VREFINT_COMP2_A, O>;
impl<'a, const O: u8> ENBUF_VREFINT_COMP2_W<'a, O> {
    #[doc = "Disables the buffer used to generate VREFINT references for COMP2"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENBUF_VREFINT_COMP2_A::Disabled)
    }
    #[doc = "Enables the buffer used to generate VREFINT references for COMP2"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENBUF_VREFINT_COMP2_A::Enabled)
    }
}
#[doc = "Field `ENREF_HSI48` reader - VREFINT reference for HSI48 oscillator enable bit"]
pub type ENREF_HSI48_R = crate::BitReader<bool>;
#[doc = "Field `ENREF_HSI48` writer - VREFINT reference for HSI48 oscillator enable bit"]
pub type ENREF_HSI48_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR3_SPEC, bool, O>;
#[doc = "Field `VREFINT_RDYF` reader - VREFINT ready flag"]
pub type VREFINT_RDYF_R = crate::BitReader<VREFINT_RDYF_A>;
#[doc = "VREFINT ready flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VREFINT_RDYF_A {
    #[doc = "0: VREFINT OFF"]
    NotReady = 0,
    #[doc = "1: VREFINT ready"]
    Ready = 1,
}
impl From<VREFINT_RDYF_A> for bool {
    #[inline(always)]
    fn from(variant: VREFINT_RDYF_A) -> Self {
        variant as u8 != 0
    }
}
impl VREFINT_RDYF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VREFINT_RDYF_A {
        match self.bits {
            false => VREFINT_RDYF_A::NotReady,
            true => VREFINT_RDYF_A::Ready,
        }
    }
    #[doc = "Checks if the value of the field is `NotReady`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == VREFINT_RDYF_A::NotReady
    }
    #[doc = "Checks if the value of the field is `Ready`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == VREFINT_RDYF_A::Ready
    }
}
#[doc = "Field `REF_LOCK` reader - SYSCFG_CFGR3 lock bit"]
pub type REF_LOCK_R = crate::BitReader<REF_LOCK_A>;
#[doc = "SYSCFG_CFGR3 lock bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REF_LOCK_A {
    #[doc = "0: SYSCFG_CFGR3\\[31:0\\]
bits are read/write"]
    ReadWrite = 0,
    #[doc = "1: SYSCFG_CFGR3\\[31:0\\]
bits are read-only"]
    ReadOnly = 1,
}
impl From<REF_LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: REF_LOCK_A) -> Self {
        variant as u8 != 0
    }
}
impl REF_LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REF_LOCK_A {
        match self.bits {
            false => REF_LOCK_A::ReadWrite,
            true => REF_LOCK_A::ReadOnly,
        }
    }
    #[doc = "Checks if the value of the field is `ReadWrite`"]
    #[inline(always)]
    pub fn is_read_write(&self) -> bool {
        *self == REF_LOCK_A::ReadWrite
    }
    #[doc = "Checks if the value of the field is `ReadOnly`"]
    #[inline(always)]
    pub fn is_read_only(&self) -> bool {
        *self == REF_LOCK_A::ReadOnly
    }
}
#[doc = "Field `REF_LOCK` writer - SYSCFG_CFGR3 lock bit"]
pub type REF_LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR3_SPEC, REF_LOCK_A, O>;
impl<'a, const O: u8> REF_LOCK_W<'a, O> {
    #[doc = "SYSCFG_CFGR3\\[31:0\\]
bits are read/write"]
    #[inline(always)]
    pub fn read_write(self) -> &'a mut W {
        self.variant(REF_LOCK_A::ReadWrite)
    }
    #[doc = "SYSCFG_CFGR3\\[31:0\\]
bits are read-only"]
    #[inline(always)]
    pub fn read_only(self) -> &'a mut W {
        self.variant(REF_LOCK_A::ReadOnly)
    }
}
impl R {
    #[doc = "Bit 0 - VREFINT enable and scaler control for COMP2 enable bit"]
    #[inline(always)]
    pub fn en_vrefint(&self) -> EN_VREFINT_R {
        EN_VREFINT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5 - BGAP_ADC connection bit"]
    #[inline(always)]
    pub fn sel_vref_out(&self) -> SEL_VREF_OUT_R {
        SEL_VREF_OUT_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - VREFINT reference for ADC enable bit"]
    #[inline(always)]
    pub fn enbuf_vrefint_adc(&self) -> ENBUF_VREFINT_ADC_R {
        ENBUF_VREFINT_ADC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Sensor reference for ADC enable bit"]
    #[inline(always)]
    pub fn enbuf_sensor_adc(&self) -> ENBUF_SENSOR_ADC_R {
        ENBUF_SENSOR_ADC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - VREFINT reference for COMP2 scaler enable bit"]
    #[inline(always)]
    pub fn enbuf_vrefint_comp2(&self) -> ENBUF_VREFINT_COMP2_R {
        ENBUF_VREFINT_COMP2_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - VREFINT reference for HSI48 oscillator enable bit"]
    #[inline(always)]
    pub fn enref_hsi48(&self) -> ENREF_HSI48_R {
        ENREF_HSI48_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 30 - VREFINT ready flag"]
    #[inline(always)]
    pub fn vrefint_rdyf(&self) -> VREFINT_RDYF_R {
        VREFINT_RDYF_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - SYSCFG_CFGR3 lock bit"]
    #[inline(always)]
    pub fn ref_lock(&self) -> REF_LOCK_R {
        REF_LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VREFINT enable and scaler control for COMP2 enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn en_vrefint(&mut self) -> EN_VREFINT_W<0> {
        EN_VREFINT_W::new(self)
    }
    #[doc = "Bits 4:5 - BGAP_ADC connection bit"]
    #[inline(always)]
    #[must_use]
    pub fn sel_vref_out(&mut self) -> SEL_VREF_OUT_W<4> {
        SEL_VREF_OUT_W::new(self)
    }
    #[doc = "Bit 8 - VREFINT reference for ADC enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn enbuf_vrefint_adc(&mut self) -> ENBUF_VREFINT_ADC_W<8> {
        ENBUF_VREFINT_ADC_W::new(self)
    }
    #[doc = "Bit 9 - Sensor reference for ADC enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn enbuf_sensor_adc(&mut self) -> ENBUF_SENSOR_ADC_W<9> {
        ENBUF_SENSOR_ADC_W::new(self)
    }
    #[doc = "Bit 12 - VREFINT reference for COMP2 scaler enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn enbuf_vrefint_comp2(&mut self) -> ENBUF_VREFINT_COMP2_W<12> {
        ENBUF_VREFINT_COMP2_W::new(self)
    }
    #[doc = "Bit 13 - VREFINT reference for HSI48 oscillator enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn enref_hsi48(&mut self) -> ENREF_HSI48_W<13> {
        ENREF_HSI48_W::new(self)
    }
    #[doc = "Bit 31 - SYSCFG_CFGR3 lock bit"]
    #[inline(always)]
    #[must_use]
    pub fn ref_lock(&mut self) -> REF_LOCK_W<31> {
        REF_LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYSCFG configuration register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr3](index.html) module"]
pub struct CFGR3_SPEC;
impl crate::RegisterSpec for CFGR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr3::R](R) reader structure"]
impl crate::Readable for CFGR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr3::W](W) writer structure"]
impl crate::Writable for CFGR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFGR3 to value 0"]
impl crate::Resettable for CFGR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
