#[doc = "Register `CFGR1` reader"]
pub struct R(crate::R<CFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR1` writer"]
pub struct W(crate::W<CFGR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR1_SPEC>;
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
impl From<crate::W<CFGR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MEM_MODE` reader - Memory mapping selection bits"]
pub type MEM_MODE_R = crate::FieldReader<u8, MEM_MODE_A>;
#[doc = "Memory mapping selection bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MEM_MODE_A {
    #[doc = "0: Main Flash memory mapped at 0x0000_0000"]
    MainFlash = 0,
    #[doc = "1: System Flash memory mapped at 0x0000_0000"]
    SystemFlash = 1,
    #[doc = "3: Embedded SRAM mapped at 0x0000_0000"]
    Sram = 3,
}
impl From<MEM_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MEM_MODE_A) -> Self {
        variant as _
    }
}
impl MEM_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MEM_MODE_A> {
        match self.bits {
            0 => Some(MEM_MODE_A::MainFlash),
            1 => Some(MEM_MODE_A::SystemFlash),
            3 => Some(MEM_MODE_A::Sram),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MainFlash`"]
    #[inline(always)]
    pub fn is_main_flash(&self) -> bool {
        *self == MEM_MODE_A::MainFlash
    }
    #[doc = "Checks if the value of the field is `SystemFlash`"]
    #[inline(always)]
    pub fn is_system_flash(&self) -> bool {
        *self == MEM_MODE_A::SystemFlash
    }
    #[doc = "Checks if the value of the field is `Sram`"]
    #[inline(always)]
    pub fn is_sram(&self) -> bool {
        *self == MEM_MODE_A::Sram
    }
}
#[doc = "Field `MEM_MODE` writer - Memory mapping selection bits"]
pub type MEM_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CFGR1_SPEC, u8, MEM_MODE_A, 2, O>;
impl<'a, const O: u8> MEM_MODE_W<'a, O> {
    #[doc = "Main Flash memory mapped at 0x0000_0000"]
    #[inline(always)]
    pub fn main_flash(self) -> &'a mut W {
        self.variant(MEM_MODE_A::MainFlash)
    }
    #[doc = "System Flash memory mapped at 0x0000_0000"]
    #[inline(always)]
    pub fn system_flash(self) -> &'a mut W {
        self.variant(MEM_MODE_A::SystemFlash)
    }
    #[doc = "Embedded SRAM mapped at 0x0000_0000"]
    #[inline(always)]
    pub fn sram(self) -> &'a mut W {
        self.variant(MEM_MODE_A::Sram)
    }
}
#[doc = "Field `UFB` reader - User bank swapping"]
pub type UFB_R = crate::BitReader<UFB_A>;
#[doc = "User bank swapping\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UFB_A {
    #[doc = "0: Flash Program memory Bank 1 is mapped at 0x0800 0000 (and aliased at 0x0000 0000 if MEM_MODE=00) and Data EEPROM Bank 1 at 0x0808 0000 (aliased at 0x0008 0000 if MEM_MODE=00)"]
    Bank1 = 0,
    #[doc = "1: Flash Program memory Bank 2 is mapped at 0x0800 0000 (and aliased at 0x0000 0000 if MEM_MODE=00) and Data EEPROM Bank 2 at 0x0808 0000 (and aliased at 0x0008 0000 if MEM_MODE=00)"]
    Bank2 = 1,
}
impl From<UFB_A> for bool {
    #[inline(always)]
    fn from(variant: UFB_A) -> Self {
        variant as u8 != 0
    }
}
impl UFB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UFB_A {
        match self.bits {
            false => UFB_A::Bank1,
            true => UFB_A::Bank2,
        }
    }
    #[doc = "Checks if the value of the field is `Bank1`"]
    #[inline(always)]
    pub fn is_bank1(&self) -> bool {
        *self == UFB_A::Bank1
    }
    #[doc = "Checks if the value of the field is `Bank2`"]
    #[inline(always)]
    pub fn is_bank2(&self) -> bool {
        *self == UFB_A::Bank2
    }
}
#[doc = "Field `UFB` writer - User bank swapping"]
pub type UFB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, UFB_A, O>;
impl<'a, const O: u8> UFB_W<'a, O> {
    #[doc = "Flash Program memory Bank 1 is mapped at 0x0800 0000 (and aliased at 0x0000 0000 if MEM_MODE=00) and Data EEPROM Bank 1 at 0x0808 0000 (aliased at 0x0008 0000 if MEM_MODE=00)"]
    #[inline(always)]
    pub fn bank1(self) -> &'a mut W {
        self.variant(UFB_A::Bank1)
    }
    #[doc = "Flash Program memory Bank 2 is mapped at 0x0800 0000 (and aliased at 0x0000 0000 if MEM_MODE=00) and Data EEPROM Bank 2 at 0x0808 0000 (and aliased at 0x0008 0000 if MEM_MODE=00)"]
    #[inline(always)]
    pub fn bank2(self) -> &'a mut W {
        self.variant(UFB_A::Bank2)
    }
}
#[doc = "Field `BOOT_MODE` reader - Boot mode selected by the boot pins status bits"]
pub type BOOT_MODE_R = crate::FieldReader<u8, BOOT_MODE_A>;
#[doc = "Boot mode selected by the boot pins status bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BOOT_MODE_A {
    #[doc = "0: Main Flash memory boot mode"]
    MainFlash = 0,
    #[doc = "1: System Flash memory boot mode"]
    SystemFlash = 1,
    #[doc = "3: Embedded SRAM boot mode"]
    Sram = 3,
}
impl From<BOOT_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: BOOT_MODE_A) -> Self {
        variant as _
    }
}
impl BOOT_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BOOT_MODE_A> {
        match self.bits {
            0 => Some(BOOT_MODE_A::MainFlash),
            1 => Some(BOOT_MODE_A::SystemFlash),
            3 => Some(BOOT_MODE_A::Sram),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MainFlash`"]
    #[inline(always)]
    pub fn is_main_flash(&self) -> bool {
        *self == BOOT_MODE_A::MainFlash
    }
    #[doc = "Checks if the value of the field is `SystemFlash`"]
    #[inline(always)]
    pub fn is_system_flash(&self) -> bool {
        *self == BOOT_MODE_A::SystemFlash
    }
    #[doc = "Checks if the value of the field is `Sram`"]
    #[inline(always)]
    pub fn is_sram(&self) -> bool {
        *self == BOOT_MODE_A::Sram
    }
}
impl R {
    #[doc = "Bits 0:1 - Memory mapping selection bits"]
    #[inline(always)]
    pub fn mem_mode(&self) -> MEM_MODE_R {
        MEM_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - User bank swapping"]
    #[inline(always)]
    pub fn ufb(&self) -> UFB_R {
        UFB_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Boot mode selected by the boot pins status bits"]
    #[inline(always)]
    pub fn boot_mode(&self) -> BOOT_MODE_R {
        BOOT_MODE_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Memory mapping selection bits"]
    #[inline(always)]
    #[must_use]
    pub fn mem_mode(&mut self) -> MEM_MODE_W<0> {
        MEM_MODE_W::new(self)
    }
    #[doc = "Bit 3 - User bank swapping"]
    #[inline(always)]
    #[must_use]
    pub fn ufb(&mut self) -> UFB_W<3> {
        UFB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYSCFG configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr1](index.html) module"]
pub struct CFGR1_SPEC;
impl crate::RegisterSpec for CFGR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr1::R](R) reader structure"]
impl crate::Readable for CFGR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr1::W](W) writer structure"]
impl crate::Writable for CFGR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFGR1 to value 0"]
impl crate::Resettable for CFGR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
