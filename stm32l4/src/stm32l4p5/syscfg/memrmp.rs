#[doc = "Register `MEMRMP` reader"]
pub struct R(crate::R<MEMRMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEMRMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEMRMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEMRMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MEMRMP` writer"]
pub struct W(crate::W<MEMRMP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MEMRMP_SPEC>;
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
impl From<crate::W<MEMRMP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MEMRMP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MEM_MODE` reader - Memory mapping selection"]
pub type MEM_MODE_R = crate::FieldReader<u8, MEM_MODE_A>;
#[doc = "Memory mapping selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MEM_MODE_A {
    #[doc = "0: Main Flash memory mapped at 0x00000000"]
    MainFlash = 0,
    #[doc = "1: System Flash memory mapped at 0x00000000"]
    SystemFlash = 1,
    #[doc = "2: FMC bank 1 (NOR/PSRAM 1 and 2) mapped at 0x00000000"]
    Fmc = 2,
    #[doc = "3: SRAM1 mapped at 0x00000000"]
    Sram1 = 3,
    #[doc = "4: OCTOSPI1 memory mapped at 0x00000000"]
    Octospi1 = 4,
    #[doc = "5: OCTOSPI2 memory mapped at 0x00000000"]
    Octospi2 = 5,
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
            2 => Some(MEM_MODE_A::Fmc),
            3 => Some(MEM_MODE_A::Sram1),
            4 => Some(MEM_MODE_A::Octospi1),
            5 => Some(MEM_MODE_A::Octospi2),
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
    #[doc = "Checks if the value of the field is `Fmc`"]
    #[inline(always)]
    pub fn is_fmc(&self) -> bool {
        *self == MEM_MODE_A::Fmc
    }
    #[doc = "Checks if the value of the field is `Sram1`"]
    #[inline(always)]
    pub fn is_sram1(&self) -> bool {
        *self == MEM_MODE_A::Sram1
    }
    #[doc = "Checks if the value of the field is `Octospi1`"]
    #[inline(always)]
    pub fn is_octospi1(&self) -> bool {
        *self == MEM_MODE_A::Octospi1
    }
    #[doc = "Checks if the value of the field is `Octospi2`"]
    #[inline(always)]
    pub fn is_octospi2(&self) -> bool {
        *self == MEM_MODE_A::Octospi2
    }
}
#[doc = "Field `MEM_MODE` writer - Memory mapping selection"]
pub type MEM_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MEMRMP_SPEC, u8, MEM_MODE_A, 3, O>;
impl<'a, const O: u8> MEM_MODE_W<'a, O> {
    #[doc = "Main Flash memory mapped at 0x00000000"]
    #[inline(always)]
    pub fn main_flash(self) -> &'a mut W {
        self.variant(MEM_MODE_A::MainFlash)
    }
    #[doc = "System Flash memory mapped at 0x00000000"]
    #[inline(always)]
    pub fn system_flash(self) -> &'a mut W {
        self.variant(MEM_MODE_A::SystemFlash)
    }
    #[doc = "FMC bank 1 (NOR/PSRAM 1 and 2) mapped at 0x00000000"]
    #[inline(always)]
    pub fn fmc(self) -> &'a mut W {
        self.variant(MEM_MODE_A::Fmc)
    }
    #[doc = "SRAM1 mapped at 0x00000000"]
    #[inline(always)]
    pub fn sram1(self) -> &'a mut W {
        self.variant(MEM_MODE_A::Sram1)
    }
    #[doc = "OCTOSPI1 memory mapped at 0x00000000"]
    #[inline(always)]
    pub fn octospi1(self) -> &'a mut W {
        self.variant(MEM_MODE_A::Octospi1)
    }
    #[doc = "OCTOSPI2 memory mapped at 0x00000000"]
    #[inline(always)]
    pub fn octospi2(self) -> &'a mut W {
        self.variant(MEM_MODE_A::Octospi2)
    }
}
#[doc = "Field `FB_MODE` reader - Flash Bank mode selection"]
pub type FB_MODE_R = crate::BitReader<FB_MODE_A>;
#[doc = "Flash Bank mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FB_MODE_A {
    #[doc = "0: Flash Bank 1 mapped at 0x0800 0000 (and aliased @0x0000 0000(1)) and Flash Bank 2 mapped at offset"]
    Normal = 0,
    #[doc = "1: Flash Bank 2 mapped at 0x0800 0000 (and aliased @0x0000 0000(1)) and Flash Bank 1 mapped at offset"]
    Inverted = 1,
}
impl From<FB_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: FB_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl FB_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FB_MODE_A {
        match self.bits {
            false => FB_MODE_A::Normal,
            true => FB_MODE_A::Inverted,
        }
    }
    #[doc = "Checks if the value of the field is `Normal`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == FB_MODE_A::Normal
    }
    #[doc = "Checks if the value of the field is `Inverted`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == FB_MODE_A::Inverted
    }
}
#[doc = "Field `FB_MODE` writer - Flash Bank mode selection"]
pub type FB_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MEMRMP_SPEC, FB_MODE_A, O>;
impl<'a, const O: u8> FB_MODE_W<'a, O> {
    #[doc = "Flash Bank 1 mapped at 0x0800 0000 (and aliased @0x0000 0000(1)) and Flash Bank 2 mapped at offset"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(FB_MODE_A::Normal)
    }
    #[doc = "Flash Bank 2 mapped at 0x0800 0000 (and aliased @0x0000 0000(1)) and Flash Bank 1 mapped at offset"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(FB_MODE_A::Inverted)
    }
}
impl R {
    #[doc = "Bits 0:2 - Memory mapping selection"]
    #[inline(always)]
    pub fn mem_mode(&self) -> MEM_MODE_R {
        MEM_MODE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - Flash Bank mode selection"]
    #[inline(always)]
    pub fn fb_mode(&self) -> FB_MODE_R {
        FB_MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Memory mapping selection"]
    #[inline(always)]
    #[must_use]
    pub fn mem_mode(&mut self) -> MEM_MODE_W<0> {
        MEM_MODE_W::new(self)
    }
    #[doc = "Bit 8 - Flash Bank mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn fb_mode(&mut self) -> FB_MODE_W<8> {
        FB_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "memory remap register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [memrmp](index.html) module"]
pub struct MEMRMP_SPEC;
impl crate::RegisterSpec for MEMRMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [memrmp::R](R) reader structure"]
impl crate::Readable for MEMRMP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [memrmp::W](W) writer structure"]
impl crate::Writable for MEMRMP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MEMRMP to value 0"]
impl crate::Resettable for MEMRMP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
