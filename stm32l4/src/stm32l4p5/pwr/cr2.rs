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
    #[doc = "0: Power voltage detector disabled"]
    Disabled = 0,
    #[doc = "1: Power voltage detector enabled"]
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
    #[doc = "Power voltage detector disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PVDE_A::Disabled)
    }
    #[doc = "Power voltage detector enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PVDE_A::Enabled)
    }
}
#[doc = "Field `PLS` reader - Power voltage detector level selection"]
pub type PLS_R = crate::FieldReader<u8, PLS_A>;
#[doc = "Power voltage detector level selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLS_A {
    #[doc = "0: VPVD0 around 2.0 V"]
    Vpvd0 = 0,
    #[doc = "1: VPVD1 around 2.2 V"]
    Vpvd1 = 1,
    #[doc = "2: VPVD2 around 2.4 V"]
    Vpvd2 = 2,
    #[doc = "3: VPVD3 around 2.5 V"]
    Vpvd3 = 3,
    #[doc = "4: VPVD4 around 2.6 V"]
    Vpvd4 = 4,
    #[doc = "5: VPVD5 around 2.8 V"]
    Vpvd5 = 5,
    #[doc = "6: VPVD6 around 2.9 V"]
    Vpvd6 = 6,
    #[doc = "7: External input analog voltage PVD_IN (compared internally to VREFINT)"]
    Pvdin = 7,
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
            0 => PLS_A::Vpvd0,
            1 => PLS_A::Vpvd1,
            2 => PLS_A::Vpvd2,
            3 => PLS_A::Vpvd3,
            4 => PLS_A::Vpvd4,
            5 => PLS_A::Vpvd5,
            6 => PLS_A::Vpvd6,
            7 => PLS_A::Pvdin,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Vpvd0`"]
    #[inline(always)]
    pub fn is_vpvd0(&self) -> bool {
        *self == PLS_A::Vpvd0
    }
    #[doc = "Checks if the value of the field is `Vpvd1`"]
    #[inline(always)]
    pub fn is_vpvd1(&self) -> bool {
        *self == PLS_A::Vpvd1
    }
    #[doc = "Checks if the value of the field is `Vpvd2`"]
    #[inline(always)]
    pub fn is_vpvd2(&self) -> bool {
        *self == PLS_A::Vpvd2
    }
    #[doc = "Checks if the value of the field is `Vpvd3`"]
    #[inline(always)]
    pub fn is_vpvd3(&self) -> bool {
        *self == PLS_A::Vpvd3
    }
    #[doc = "Checks if the value of the field is `Vpvd4`"]
    #[inline(always)]
    pub fn is_vpvd4(&self) -> bool {
        *self == PLS_A::Vpvd4
    }
    #[doc = "Checks if the value of the field is `Vpvd5`"]
    #[inline(always)]
    pub fn is_vpvd5(&self) -> bool {
        *self == PLS_A::Vpvd5
    }
    #[doc = "Checks if the value of the field is `Vpvd6`"]
    #[inline(always)]
    pub fn is_vpvd6(&self) -> bool {
        *self == PLS_A::Vpvd6
    }
    #[doc = "Checks if the value of the field is `Pvdin`"]
    #[inline(always)]
    pub fn is_pvdin(&self) -> bool {
        *self == PLS_A::Pvdin
    }
}
#[doc = "Field `PLS` writer - Power voltage detector level selection"]
pub type PLS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CR2_SPEC, u8, PLS_A, 3, O>;
impl<'a, const O: u8> PLS_W<'a, O> {
    #[doc = "VPVD0 around 2.0 V"]
    #[inline(always)]
    pub fn vpvd0(self) -> &'a mut W {
        self.variant(PLS_A::Vpvd0)
    }
    #[doc = "VPVD1 around 2.2 V"]
    #[inline(always)]
    pub fn vpvd1(self) -> &'a mut W {
        self.variant(PLS_A::Vpvd1)
    }
    #[doc = "VPVD2 around 2.4 V"]
    #[inline(always)]
    pub fn vpvd2(self) -> &'a mut W {
        self.variant(PLS_A::Vpvd2)
    }
    #[doc = "VPVD3 around 2.5 V"]
    #[inline(always)]
    pub fn vpvd3(self) -> &'a mut W {
        self.variant(PLS_A::Vpvd3)
    }
    #[doc = "VPVD4 around 2.6 V"]
    #[inline(always)]
    pub fn vpvd4(self) -> &'a mut W {
        self.variant(PLS_A::Vpvd4)
    }
    #[doc = "VPVD5 around 2.8 V"]
    #[inline(always)]
    pub fn vpvd5(self) -> &'a mut W {
        self.variant(PLS_A::Vpvd5)
    }
    #[doc = "VPVD6 around 2.9 V"]
    #[inline(always)]
    pub fn vpvd6(self) -> &'a mut W {
        self.variant(PLS_A::Vpvd6)
    }
    #[doc = "External input analog voltage PVD_IN (compared internally to VREFINT)"]
    #[inline(always)]
    pub fn pvdin(self) -> &'a mut W {
        self.variant(PLS_A::Pvdin)
    }
}
#[doc = "Field `PVME1` reader - Peripheral voltage monitoring 1 enable: VDDUSB vs. 1.2V"]
pub type PVME1_R = crate::BitReader<PVME1_A>;
#[doc = "Peripheral voltage monitoring 1 enable: VDDUSB vs. 1.2V\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVME1_A {
    #[doc = "0: PVM2 (VDDUSB monitoring vs. 1.2V threshold) disable"]
    Disabled = 0,
    #[doc = "1: PVM2 (VDDUSB monitoring vs. 1.2V threshold) enable"]
    Enabled = 1,
}
impl From<PVME1_A> for bool {
    #[inline(always)]
    fn from(variant: PVME1_A) -> Self {
        variant as u8 != 0
    }
}
impl PVME1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PVME1_A {
        match self.bits {
            false => PVME1_A::Disabled,
            true => PVME1_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PVME1_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PVME1_A::Enabled
    }
}
#[doc = "Field `PVME1` writer - Peripheral voltage monitoring 1 enable: VDDUSB vs. 1.2V"]
pub type PVME1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, PVME1_A, O>;
impl<'a, const O: u8> PVME1_W<'a, O> {
    #[doc = "PVM2 (VDDUSB monitoring vs. 1.2V threshold) disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PVME1_A::Disabled)
    }
    #[doc = "PVM2 (VDDUSB monitoring vs. 1.2V threshold) enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PVME1_A::Enabled)
    }
}
#[doc = "Field `PVME2` reader - Peripheral voltage monitoring 2 enable: VDDIO2 vs. 0.9V"]
pub type PVME2_R = crate::BitReader<PVME2_A>;
#[doc = "Peripheral voltage monitoring 2 enable: VDDIO2 vs. 0.9V\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVME2_A {
    #[doc = "0: PVM2 (VDDIO2 monitoring vs. 0.9V threshold) disable"]
    Disabled = 0,
    #[doc = "1: PVM2 (VDDIO2 monitoring vs. 0.9V threshold) enable"]
    Enabled = 1,
}
impl From<PVME2_A> for bool {
    #[inline(always)]
    fn from(variant: PVME2_A) -> Self {
        variant as u8 != 0
    }
}
impl PVME2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PVME2_A {
        match self.bits {
            false => PVME2_A::Disabled,
            true => PVME2_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PVME2_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PVME2_A::Enabled
    }
}
#[doc = "Field `PVME2` writer - Peripheral voltage monitoring 2 enable: VDDIO2 vs. 0.9V"]
pub type PVME2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, PVME2_A, O>;
impl<'a, const O: u8> PVME2_W<'a, O> {
    #[doc = "PVM2 (VDDIO2 monitoring vs. 0.9V threshold) disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PVME2_A::Disabled)
    }
    #[doc = "PVM2 (VDDIO2 monitoring vs. 0.9V threshold) enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PVME2_A::Enabled)
    }
}
#[doc = "Field `PVME3` reader - Peripheral voltage monitoring 3 enable: VDDA vs. 1.62V"]
pub type PVME3_R = crate::BitReader<PVME3_A>;
#[doc = "Peripheral voltage monitoring 3 enable: VDDA vs. 1.62V\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVME3_A {
    #[doc = "0: PVM3 (VDDA monitoring vs. 1.62V threshold) disable"]
    Disabled = 0,
    #[doc = "1: PVM3 (VDDA monitoring vs. 1.62V threshold) enable"]
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
    #[doc = "PVM3 (VDDA monitoring vs. 1.62V threshold) disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PVME3_A::Disabled)
    }
    #[doc = "PVM3 (VDDA monitoring vs. 1.62V threshold) enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PVME3_A::Enabled)
    }
}
#[doc = "Field `PVME4` reader - Peripheral voltage monitoring 4 enable: VDDA vs. 2.2V"]
pub type PVME4_R = crate::BitReader<PVME4_A>;
#[doc = "Peripheral voltage monitoring 4 enable: VDDA vs. 2.2V\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVME4_A {
    #[doc = "0: PVM4 (VDDA monitoring vs. 2.2V threshold) disable"]
    Disabled = 0,
    #[doc = "1: PVM4 (VDDA monitoring vs. 2.2V threshold) enable"]
    Enabled = 1,
}
impl From<PVME4_A> for bool {
    #[inline(always)]
    fn from(variant: PVME4_A) -> Self {
        variant as u8 != 0
    }
}
impl PVME4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PVME4_A {
        match self.bits {
            false => PVME4_A::Disabled,
            true => PVME4_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PVME4_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PVME4_A::Enabled
    }
}
#[doc = "Field `PVME4` writer - Peripheral voltage monitoring 4 enable: VDDA vs. 2.2V"]
pub type PVME4_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, PVME4_A, O>;
impl<'a, const O: u8> PVME4_W<'a, O> {
    #[doc = "PVM4 (VDDA monitoring vs. 2.2V threshold) disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PVME4_A::Disabled)
    }
    #[doc = "PVM4 (VDDA monitoring vs. 2.2V threshold) enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PVME4_A::Enabled)
    }
}
#[doc = "Field `IOSV` reader - VDDIO2 Independent I/Os supply valid"]
pub type IOSV_R = crate::BitReader<IOSV_A>;
#[doc = "VDDIO2 Independent I/Os supply valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOSV_A {
    #[doc = "0: VDDIO2 is not present. Logical and electrical isolation is applied to ignore this supply"]
    NotPresent = 0,
    #[doc = "1: VDDIO2 is valid"]
    Valid = 1,
}
impl From<IOSV_A> for bool {
    #[inline(always)]
    fn from(variant: IOSV_A) -> Self {
        variant as u8 != 0
    }
}
impl IOSV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOSV_A {
        match self.bits {
            false => IOSV_A::NotPresent,
            true => IOSV_A::Valid,
        }
    }
    #[doc = "Checks if the value of the field is `NotPresent`"]
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        *self == IOSV_A::NotPresent
    }
    #[doc = "Checks if the value of the field is `Valid`"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == IOSV_A::Valid
    }
}
#[doc = "Field `IOSV` writer - VDDIO2 Independent I/Os supply valid"]
pub type IOSV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, IOSV_A, O>;
impl<'a, const O: u8> IOSV_W<'a, O> {
    #[doc = "VDDIO2 is not present. Logical and electrical isolation is applied to ignore this supply"]
    #[inline(always)]
    pub fn not_present(self) -> &'a mut W {
        self.variant(IOSV_A::NotPresent)
    }
    #[doc = "VDDIO2 is valid"]
    #[inline(always)]
    pub fn valid(self) -> &'a mut W {
        self.variant(IOSV_A::Valid)
    }
}
#[doc = "Field `USV` reader - VDDUSB USB supply valid"]
pub type USV_R = crate::BitReader<USV_A>;
#[doc = "VDDUSB USB supply valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USV_A {
    #[doc = "0: VDDUSB is not present. Logical and electrical isolation is applied to ignore this supply"]
    NotPresent = 0,
    #[doc = "1: VDDUSB is valid"]
    Valid = 1,
}
impl From<USV_A> for bool {
    #[inline(always)]
    fn from(variant: USV_A) -> Self {
        variant as u8 != 0
    }
}
impl USV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USV_A {
        match self.bits {
            false => USV_A::NotPresent,
            true => USV_A::Valid,
        }
    }
    #[doc = "Checks if the value of the field is `NotPresent`"]
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        *self == USV_A::NotPresent
    }
    #[doc = "Checks if the value of the field is `Valid`"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == USV_A::Valid
    }
}
#[doc = "Field `USV` writer - VDDUSB USB supply valid"]
pub type USV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, USV_A, O>;
impl<'a, const O: u8> USV_W<'a, O> {
    #[doc = "VDDUSB is not present. Logical and electrical isolation is applied to ignore this supply"]
    #[inline(always)]
    pub fn not_present(self) -> &'a mut W {
        self.variant(USV_A::NotPresent)
    }
    #[doc = "VDDUSB is valid"]
    #[inline(always)]
    pub fn valid(self) -> &'a mut W {
        self.variant(USV_A::Valid)
    }
}
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
    #[doc = "Bit 4 - Peripheral voltage monitoring 1 enable: VDDUSB vs. 1.2V"]
    #[inline(always)]
    pub fn pvme1(&self) -> PVME1_R {
        PVME1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Peripheral voltage monitoring 2 enable: VDDIO2 vs. 0.9V"]
    #[inline(always)]
    pub fn pvme2(&self) -> PVME2_R {
        PVME2_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Peripheral voltage monitoring 3 enable: VDDA vs. 1.62V"]
    #[inline(always)]
    pub fn pvme3(&self) -> PVME3_R {
        PVME3_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Peripheral voltage monitoring 4 enable: VDDA vs. 2.2V"]
    #[inline(always)]
    pub fn pvme4(&self) -> PVME4_R {
        PVME4_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - VDDIO2 Independent I/Os supply valid"]
    #[inline(always)]
    pub fn iosv(&self) -> IOSV_R {
        IOSV_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - VDDUSB USB supply valid"]
    #[inline(always)]
    pub fn usv(&self) -> USV_R {
        USV_R::new(((self.bits >> 10) & 1) != 0)
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
    #[doc = "Bit 4 - Peripheral voltage monitoring 1 enable: VDDUSB vs. 1.2V"]
    #[inline(always)]
    #[must_use]
    pub fn pvme1(&mut self) -> PVME1_W<4> {
        PVME1_W::new(self)
    }
    #[doc = "Bit 5 - Peripheral voltage monitoring 2 enable: VDDIO2 vs. 0.9V"]
    #[inline(always)]
    #[must_use]
    pub fn pvme2(&mut self) -> PVME2_W<5> {
        PVME2_W::new(self)
    }
    #[doc = "Bit 6 - Peripheral voltage monitoring 3 enable: VDDA vs. 1.62V"]
    #[inline(always)]
    #[must_use]
    pub fn pvme3(&mut self) -> PVME3_W<6> {
        PVME3_W::new(self)
    }
    #[doc = "Bit 7 - Peripheral voltage monitoring 4 enable: VDDA vs. 2.2V"]
    #[inline(always)]
    #[must_use]
    pub fn pvme4(&mut self) -> PVME4_W<7> {
        PVME4_W::new(self)
    }
    #[doc = "Bit 9 - VDDIO2 Independent I/Os supply valid"]
    #[inline(always)]
    #[must_use]
    pub fn iosv(&mut self) -> IOSV_W<9> {
        IOSV_W::new(self)
    }
    #[doc = "Bit 10 - VDDUSB USB supply valid"]
    #[inline(always)]
    #[must_use]
    pub fn usv(&mut self) -> USV_W<10> {
        USV_W::new(self)
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
