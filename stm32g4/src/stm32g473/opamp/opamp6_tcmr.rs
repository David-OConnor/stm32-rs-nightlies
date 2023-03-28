#[doc = "Register `OPAMP6_TCMR` reader"]
pub struct R(crate::R<OPAMP6_TCMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPAMP6_TCMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPAMP6_TCMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPAMP6_TCMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPAMP6_TCMR` writer"]
pub struct W(crate::W<OPAMP6_TCMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPAMP6_TCMR_SPEC>;
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
impl From<crate::W<OPAMP6_TCMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPAMP6_TCMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VMS_SEL` reader - VMS_SEL"]
pub type VMS_SEL_R = crate::BitReader<bool>;
#[doc = "Field `VMS_SEL` writer - VMS_SEL"]
pub type VMS_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPAMP6_TCMR_SPEC, bool, O>;
#[doc = "Field `VPS_SEL` reader - VPS_SEL"]
pub type VPS_SEL_R = crate::FieldReader<u8, VPS_SEL_A>;
#[doc = "VPS_SEL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VPS_SEL_A {
    #[doc = "0: VINP0 connected to VINP input"]
    Vinp0 = 0,
    #[doc = "1: VINP1 connected to VINP input"]
    Vinp1 = 1,
    #[doc = "2: VINP2 connected to VINP input"]
    Vinp2 = 2,
    #[doc = "3: DAC3_CH1 connected to VINP input"]
    Dac3Ch1 = 3,
}
impl From<VPS_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: VPS_SEL_A) -> Self {
        variant as _
    }
}
impl VPS_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VPS_SEL_A {
        match self.bits {
            0 => VPS_SEL_A::Vinp0,
            1 => VPS_SEL_A::Vinp1,
            2 => VPS_SEL_A::Vinp2,
            3 => VPS_SEL_A::Dac3Ch1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Vinp0`"]
    #[inline(always)]
    pub fn is_vinp0(&self) -> bool {
        *self == VPS_SEL_A::Vinp0
    }
    #[doc = "Checks if the value of the field is `Vinp1`"]
    #[inline(always)]
    pub fn is_vinp1(&self) -> bool {
        *self == VPS_SEL_A::Vinp1
    }
    #[doc = "Checks if the value of the field is `Vinp2`"]
    #[inline(always)]
    pub fn is_vinp2(&self) -> bool {
        *self == VPS_SEL_A::Vinp2
    }
    #[doc = "Checks if the value of the field is `Dac3Ch1`"]
    #[inline(always)]
    pub fn is_dac3_ch1(&self) -> bool {
        *self == VPS_SEL_A::Dac3Ch1
    }
}
#[doc = "Field `VPS_SEL` writer - VPS_SEL"]
pub type VPS_SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, OPAMP6_TCMR_SPEC, u8, VPS_SEL_A, 2, O>;
impl<'a, const O: u8> VPS_SEL_W<'a, O> {
    #[doc = "VINP0 connected to VINP input"]
    #[inline(always)]
    pub fn vinp0(self) -> &'a mut W {
        self.variant(VPS_SEL_A::Vinp0)
    }
    #[doc = "VINP1 connected to VINP input"]
    #[inline(always)]
    pub fn vinp1(self) -> &'a mut W {
        self.variant(VPS_SEL_A::Vinp1)
    }
    #[doc = "VINP2 connected to VINP input"]
    #[inline(always)]
    pub fn vinp2(self) -> &'a mut W {
        self.variant(VPS_SEL_A::Vinp2)
    }
    #[doc = "DAC3_CH1 connected to VINP input"]
    #[inline(always)]
    pub fn dac3_ch1(self) -> &'a mut W {
        self.variant(VPS_SEL_A::Dac3Ch1)
    }
}
#[doc = "Field `T1CM_EN` reader - T1CM_EN"]
pub type T1CM_EN_R = crate::BitReader<T1CM_EN_A>;
#[doc = "T1CM_EN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum T1CM_EN_A {
    #[doc = "0: Automatic input switch triggered by TIM1 disabled"]
    Disabled = 0,
    #[doc = "1: Automatic input switch triggered by TIM1 enabled"]
    Enabled = 1,
}
impl From<T1CM_EN_A> for bool {
    #[inline(always)]
    fn from(variant: T1CM_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl T1CM_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> T1CM_EN_A {
        match self.bits {
            false => T1CM_EN_A::Disabled,
            true => T1CM_EN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == T1CM_EN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == T1CM_EN_A::Enabled
    }
}
#[doc = "Field `T1CM_EN` writer - T1CM_EN"]
pub type T1CM_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPAMP6_TCMR_SPEC, T1CM_EN_A, O>;
impl<'a, const O: u8> T1CM_EN_W<'a, O> {
    #[doc = "Automatic input switch triggered by TIM1 disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(T1CM_EN_A::Disabled)
    }
    #[doc = "Automatic input switch triggered by TIM1 enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(T1CM_EN_A::Enabled)
    }
}
#[doc = "Field `T8CM_EN` reader - T8CM_EN"]
pub type T8CM_EN_R = crate::BitReader<T8CM_EN_A>;
#[doc = "T8CM_EN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum T8CM_EN_A {
    #[doc = "0: Automatic input switch triggered by TIM8 disabled"]
    Disabled = 0,
    #[doc = "1: Automatic input switch triggered by TIM8 enabled"]
    Enabled = 1,
}
impl From<T8CM_EN_A> for bool {
    #[inline(always)]
    fn from(variant: T8CM_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl T8CM_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> T8CM_EN_A {
        match self.bits {
            false => T8CM_EN_A::Disabled,
            true => T8CM_EN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == T8CM_EN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == T8CM_EN_A::Enabled
    }
}
#[doc = "Field `T8CM_EN` writer - T8CM_EN"]
pub type T8CM_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPAMP6_TCMR_SPEC, T8CM_EN_A, O>;
impl<'a, const O: u8> T8CM_EN_W<'a, O> {
    #[doc = "Automatic input switch triggered by TIM8 disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(T8CM_EN_A::Disabled)
    }
    #[doc = "Automatic input switch triggered by TIM8 enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(T8CM_EN_A::Enabled)
    }
}
#[doc = "Field `T20CM_EN` reader - T20CM_EN"]
pub type T20CM_EN_R = crate::BitReader<T20CM_EN_A>;
#[doc = "T20CM_EN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum T20CM_EN_A {
    #[doc = "0: Automatic input switch triggered by TIM20 disabled"]
    Disabled = 0,
    #[doc = "1: Automatic input switch triggered by TIM20 enabled"]
    Enabled = 1,
}
impl From<T20CM_EN_A> for bool {
    #[inline(always)]
    fn from(variant: T20CM_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl T20CM_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> T20CM_EN_A {
        match self.bits {
            false => T20CM_EN_A::Disabled,
            true => T20CM_EN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == T20CM_EN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == T20CM_EN_A::Enabled
    }
}
#[doc = "Field `T20CM_EN` writer - T20CM_EN"]
pub type T20CM_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPAMP6_TCMR_SPEC, T20CM_EN_A, O>;
impl<'a, const O: u8> T20CM_EN_W<'a, O> {
    #[doc = "Automatic input switch triggered by TIM20 disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(T20CM_EN_A::Disabled)
    }
    #[doc = "Automatic input switch triggered by TIM20 enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(T20CM_EN_A::Enabled)
    }
}
#[doc = "Field `LOCK` reader - LOCK"]
pub type LOCK_R = crate::BitReader<LOCK_A>;
#[doc = "LOCK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCK_A {
    #[doc = "0: TCMR is read-write"]
    ReadWrite = 0,
    #[doc = "1: TCMR is read-only, can only be cleared by system reset"]
    ReadOnly = 1,
}
impl From<LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK_A) -> Self {
        variant as u8 != 0
    }
}
impl LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCK_A {
        match self.bits {
            false => LOCK_A::ReadWrite,
            true => LOCK_A::ReadOnly,
        }
    }
    #[doc = "Checks if the value of the field is `ReadWrite`"]
    #[inline(always)]
    pub fn is_read_write(&self) -> bool {
        *self == LOCK_A::ReadWrite
    }
    #[doc = "Checks if the value of the field is `ReadOnly`"]
    #[inline(always)]
    pub fn is_read_only(&self) -> bool {
        *self == LOCK_A::ReadOnly
    }
}
#[doc = "Field `LOCK` writer - LOCK"]
pub type LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPAMP6_TCMR_SPEC, LOCK_A, O>;
impl<'a, const O: u8> LOCK_W<'a, O> {
    #[doc = "TCMR is read-write"]
    #[inline(always)]
    pub fn read_write(self) -> &'a mut W {
        self.variant(LOCK_A::ReadWrite)
    }
    #[doc = "TCMR is read-only, can only be cleared by system reset"]
    #[inline(always)]
    pub fn read_only(self) -> &'a mut W {
        self.variant(LOCK_A::ReadOnly)
    }
}
impl R {
    #[doc = "Bit 0 - VMS_SEL"]
    #[inline(always)]
    pub fn vms_sel(&self) -> VMS_SEL_R {
        VMS_SEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - VPS_SEL"]
    #[inline(always)]
    pub fn vps_sel(&self) -> VPS_SEL_R {
        VPS_SEL_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - T1CM_EN"]
    #[inline(always)]
    pub fn t1cm_en(&self) -> T1CM_EN_R {
        T1CM_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - T8CM_EN"]
    #[inline(always)]
    pub fn t8cm_en(&self) -> T8CM_EN_R {
        T8CM_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - T20CM_EN"]
    #[inline(always)]
    pub fn t20cm_en(&self) -> T20CM_EN_R {
        T20CM_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 31 - LOCK"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VMS_SEL"]
    #[inline(always)]
    #[must_use]
    pub fn vms_sel(&mut self) -> VMS_SEL_W<0> {
        VMS_SEL_W::new(self)
    }
    #[doc = "Bits 1:2 - VPS_SEL"]
    #[inline(always)]
    #[must_use]
    pub fn vps_sel(&mut self) -> VPS_SEL_W<1> {
        VPS_SEL_W::new(self)
    }
    #[doc = "Bit 3 - T1CM_EN"]
    #[inline(always)]
    #[must_use]
    pub fn t1cm_en(&mut self) -> T1CM_EN_W<3> {
        T1CM_EN_W::new(self)
    }
    #[doc = "Bit 4 - T8CM_EN"]
    #[inline(always)]
    #[must_use]
    pub fn t8cm_en(&mut self) -> T8CM_EN_W<4> {
        T8CM_EN_W::new(self)
    }
    #[doc = "Bit 5 - T20CM_EN"]
    #[inline(always)]
    #[must_use]
    pub fn t20cm_en(&mut self) -> T20CM_EN_W<5> {
        T20CM_EN_W::new(self)
    }
    #[doc = "Bit 31 - LOCK"]
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
#[doc = "OPAMP6 control/status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opamp6_tcmr](index.html) module"]
pub struct OPAMP6_TCMR_SPEC;
impl crate::RegisterSpec for OPAMP6_TCMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [opamp6_tcmr::R](R) reader structure"]
impl crate::Readable for OPAMP6_TCMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [opamp6_tcmr::W](W) writer structure"]
impl crate::Writable for OPAMP6_TCMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OPAMP6_TCMR to value 0"]
impl crate::Resettable for OPAMP6_TCMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
