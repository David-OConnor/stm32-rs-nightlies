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
pub type PVDE_R = crate::BitReader<PVDE_A>;
#[doc = "Power voltage detector enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVDE_A {
    #[doc = "0: PVD Disabled"]
    Disabled = 0,
    #[doc = "1: PVD Enabled"]
    Enabled = 1,
}
impl From<PVDE_A> for bool {
    #[inline(always)]
    fn from(variant: PVDE_A) -> Self {
        variant as u8 != 0
    }
}
impl PVDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PVDE_A {
        match self.bits {
            false => PVDE_A::Disabled,
            true => PVDE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PVDE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PVDE_A::Enabled
    }
}
#[doc = "Field `PVDE` writer - Power voltage detector enable"]
pub type PVDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, PVDE_A, O>;
impl<'a, const O: u8> PVDE_W<'a, O> {
    #[doc = "PVD Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PVDE_A::Disabled)
    }
    #[doc = "PVD Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PVDE_A::Enabled)
    }
}
#[doc = "Field `PLS` reader - Power voltage detector level selection."]
pub type PLS_R = crate::FieldReader<u8, PLS_A>;
#[doc = "Power voltage detector level selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLS_A {
    #[doc = "0: 2.0V"]
    V20 = 0,
    #[doc = "1: 2.2V"]
    V22 = 1,
    #[doc = "2: 2.4V"]
    V24 = 2,
    #[doc = "3: 2.5V"]
    V25 = 3,
    #[doc = "4: 2.6V"]
    V26 = 4,
    #[doc = "5: 2.8V"]
    V28 = 5,
    #[doc = "6: 2.9V"]
    V29 = 6,
    #[doc = "7: External input analog voltage PVD_IN (compared internally to VREFINT)"]
    External = 7,
}
impl From<PLS_A> for u8 {
    #[inline(always)]
    fn from(variant: PLS_A) -> Self {
        variant as _
    }
}
impl PLS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLS_A {
        match self.bits {
            0 => PLS_A::V20,
            1 => PLS_A::V22,
            2 => PLS_A::V24,
            3 => PLS_A::V25,
            4 => PLS_A::V26,
            5 => PLS_A::V28,
            6 => PLS_A::V29,
            7 => PLS_A::External,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `V20`"]
    #[inline(always)]
    pub fn is_v2_0(&self) -> bool {
        *self == PLS_A::V20
    }
    #[doc = "Checks if the value of the field is `V22`"]
    #[inline(always)]
    pub fn is_v2_2(&self) -> bool {
        *self == PLS_A::V22
    }
    #[doc = "Checks if the value of the field is `V24`"]
    #[inline(always)]
    pub fn is_v2_4(&self) -> bool {
        *self == PLS_A::V24
    }
    #[doc = "Checks if the value of the field is `V25`"]
    #[inline(always)]
    pub fn is_v2_5(&self) -> bool {
        *self == PLS_A::V25
    }
    #[doc = "Checks if the value of the field is `V26`"]
    #[inline(always)]
    pub fn is_v2_6(&self) -> bool {
        *self == PLS_A::V26
    }
    #[doc = "Checks if the value of the field is `V28`"]
    #[inline(always)]
    pub fn is_v2_8(&self) -> bool {
        *self == PLS_A::V28
    }
    #[doc = "Checks if the value of the field is `V29`"]
    #[inline(always)]
    pub fn is_v2_9(&self) -> bool {
        *self == PLS_A::V29
    }
    #[doc = "Checks if the value of the field is `External`"]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        *self == PLS_A::External
    }
}
#[doc = "Field `PLS` writer - Power voltage detector level selection."]
pub type PLS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CR2_SPEC, u8, PLS_A, 3, O>;
impl<'a, const O: u8> PLS_W<'a, O> {
    #[doc = "2.0V"]
    #[inline(always)]
    pub fn v2_0(self) -> &'a mut W {
        self.variant(PLS_A::V20)
    }
    #[doc = "2.2V"]
    #[inline(always)]
    pub fn v2_2(self) -> &'a mut W {
        self.variant(PLS_A::V22)
    }
    #[doc = "2.4V"]
    #[inline(always)]
    pub fn v2_4(self) -> &'a mut W {
        self.variant(PLS_A::V24)
    }
    #[doc = "2.5V"]
    #[inline(always)]
    pub fn v2_5(self) -> &'a mut W {
        self.variant(PLS_A::V25)
    }
    #[doc = "2.6V"]
    #[inline(always)]
    pub fn v2_6(self) -> &'a mut W {
        self.variant(PLS_A::V26)
    }
    #[doc = "2.8V"]
    #[inline(always)]
    pub fn v2_8(self) -> &'a mut W {
        self.variant(PLS_A::V28)
    }
    #[doc = "2.9V"]
    #[inline(always)]
    pub fn v2_9(self) -> &'a mut W {
        self.variant(PLS_A::V29)
    }
    #[doc = "External input analog voltage PVD_IN (compared internally to VREFINT)"]
    #[inline(always)]
    pub fn external(self) -> &'a mut W {
        self.variant(PLS_A::External)
    }
}
#[doc = "Field `PVME3` reader - Peripheral voltage monitoring 3 enable: VDDA vs. 1.62V"]
pub type PVME3_R = crate::BitReader<PVME3_A>;
#[doc = "Peripheral voltage monitoring 3 enable: VDDA vs. 1.62V\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVME3_A {
    #[doc = "0: PVM3 (VDDA monitoring versus 1.62 V threshold) disable"]
    Disabled = 0,
    #[doc = "1: PVM3 (VDDA monitoring versus 1.62 V threshold) enable"]
    Enabled = 1,
}
impl From<PVME3_A> for bool {
    #[inline(always)]
    fn from(variant: PVME3_A) -> Self {
        variant as u8 != 0
    }
}
impl PVME3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PVME3_A {
        match self.bits {
            false => PVME3_A::Disabled,
            true => PVME3_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PVME3_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PVME3_A::Enabled
    }
}
#[doc = "Field `PVME3` writer - Peripheral voltage monitoring 3 enable: VDDA vs. 1.62V"]
pub type PVME3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, PVME3_A, O>;
impl<'a, const O: u8> PVME3_W<'a, O> {
    #[doc = "PVM3 (VDDA monitoring versus 1.62 V threshold) disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PVME3_A::Disabled)
    }
    #[doc = "PVM3 (VDDA monitoring versus 1.62 V threshold) enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PVME3_A::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Power voltage detector enable"]
    #[inline(always)]
    pub fn pvde(&self) -> PVDE_R {
        PVDE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Power voltage detector level selection."]
    #[inline(always)]
    pub fn pls(&self) -> PLS_R {
        PLS_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 6 - Peripheral voltage monitoring 3 enable: VDDA vs. 1.62V"]
    #[inline(always)]
    pub fn pvme3(&self) -> PVME3_R {
        PVME3_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power voltage detector enable"]
    #[inline(always)]
    #[must_use]
    pub fn pvde(&mut self) -> PVDE_W<0> {
        PVDE_W::new(self)
    }
    #[doc = "Bits 1:3 - Power voltage detector level selection."]
    #[inline(always)]
    #[must_use]
    pub fn pls(&mut self) -> PLS_W<1> {
        PLS_W::new(self)
    }
    #[doc = "Bit 6 - Peripheral voltage monitoring 3 enable: VDDA vs. 1.62V"]
    #[inline(always)]
    #[must_use]
    pub fn pvme3(&mut self) -> PVME3_W<6> {
        PVME3_W::new(self)
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
