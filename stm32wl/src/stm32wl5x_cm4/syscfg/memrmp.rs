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
#[doc = "Field `MEM_MODE` writer - Memory mapping selection"]
pub type MEM_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MEMRMP_SPEC, u8, MEM_MODE_A, 3, O>;
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
impl R {
    #[doc = "Bits 0:2 - Memory mapping selection"]
    #[inline(always)]
    pub fn mem_mode(&self) -> MEM_MODE_R {
        MEM_MODE_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Memory mapping selection"]
    #[inline(always)]
    #[must_use]
    pub fn mem_mode(&mut self) -> MEM_MODE_W<0> {
        MEM_MODE_W::new(self)
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
